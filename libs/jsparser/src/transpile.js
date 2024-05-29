// TODO: Rewrite the parser using dfagen and lalrgen.

'use strict';

import {
  assert,
  assertEquals,
  assertExists,
  unreachable,
} from 'https://deno.land/std@0.224.0/testing/asserts.ts';
import * as log from 'https://deno.land/std@0.224.0/log/mod.ts';
import * as yaml from 'https://deno.land/std@0.224.0/yaml/mod.ts';
import * as changeCase from 'https://deno.land/x/case@2.2.0/mod.ts';
import { parseCommand, readAllText } from '../../../tools/lib/cli.js';
import { setup } from '../../../tools/lib/log.js';

const PROGNAME = 'transpile.js';

const DOC = `
Transpile an ECMA lexical grammer from the text format to the YAML format.

Usage:
  ${PROGNAME} [options]
  ${PROGNAME} -h | --help

Options:
  -d, --debug
    Enable debug logs.

  -g, --grammar-type=<grammar-type>
    The type of grammar to transpile.

  -t, --tokens=<tokens-json>
    Path to a tokens.json.

Note:
  Terminal symbols in the syntactic grammar are converted into the constant case
  so that we can easily distinguish terminal symbols from non-terminal symbols.
`.trim();

const HEADER = `
# DO NOT EDIT THIS FILE BY HAND.
#
# This file was automagically generated by ${PROGNAME}.
`.trim();

async function run(options) {
  if (options.debug) {
    setup(PROGNAME, 'DEBUG');
  } else {
    setup(PROGNAME, 'INFO');
  }
  const transpiler = new Transpiler(options);
  printYaml(await transpiler.transpile());
}

function transform(rules) {
  const result = [];
  for (const rule of rules) {
    for (const production of rule.productions) {
      assert(Array.isArray(production), rule.name);
      result.push({ name: rule.name, production });
    }
  }
  return result;
}

function convertTokenNames(rules) {
  log.debug('Convert token names in the constant case...');
  for (const rule of rules) {
    for (const term of rule.production) {
      switch (term.type) {
        case 'token':
        case 'disallow':
          term.data = changeCase.constantCase(term.data);
          break;
        case 'lookahead':
          term.data.data = term.data.data.map((patterns) => {
            return patterns.map((pattern) => {
              if (pattern.startsWith('(!')) {
                let token = pattern.substring(2, pattern.length - 1);
                return `(!${changeCase.constantCase(token)})`;
              }
              return changeCase.constantCase(pattern);
            });
          });
          break;
      }
    }
  }
  return rules;
}

function rewriteIdentifierName(rules) {
  log.debug('Rewriting rules using IdentifierName...');

  for (const rule of rules) {
    let changed = false;
    for (const term of rule.production) {
      if (term.type === 'token' && term.data === 'IdentifierName') {
        term.type = 'non-terminal';
        term.data = 'KeywordOrIdentifierName';
        changed = true;
      }
    }
    if (changed) {
      log.debug(`  Rewrite ${rule.name}`);
    }
  }

  log.debug(`  Adding IdentifierNameButNotReservedWord...`);
  rules.push({
    name: 'IdentifierNameButNotReservedWord',
    production: [{ type: 'token', data: 'IdentifierName' }],
  });
  for (const kw of ADDITIONAL_KEYWORDS) {
    rules.push({
      name: 'IdentifierNameButNotReservedWord',
      production: [{ type: 'token', data: kw.toUpperCase() }],
    });
  }

  log.debug(`  Adding KeywordOrIdentifierName...`);
  rules.push({
    name: 'KeywordOrIdentifierName',
    production: [{ type: 'token', data: 'IdentifierName' }],
  });
  for (const kw of RESERVED_KEYWORDS) {
    rules.push({
      name: 'KeywordOrIdentifierName',
      production: [{ type: 'token', data: kw.toUpperCase() }],
    });
  }
  for (const kw of ADDITIONAL_KEYWORDS) {
    rules.push({
      name: 'KeywordOrIdentifierName',
      production: [{ type: 'token', data: kw.toUpperCase() }],
    });
  }

  return rules;
}

class Transpiler {
  constructor(options) {
    this.options_ = options;
    switch (options.grammarType) {
      case 'lexical':
        this.passes_ = [
          addLexicalRules,
          rewriteReservedWord,
          rewritePunctuator,
          expandOptionals,
          expandParameterizedRules,
          translateRules,
          addSourceCharacter,
          mergeUnicodeSets,
          transform,
        ];
        break;
      case 'syntactic':
        this.passes_ = [
          rewriteIdentifierRule,
          expandMultiplicativeOperator,
          expandLetOrConst,
          // CPEAAPL cannot be replaced with refined production rules.  You will see many
          // conflicts in the LALR(1) parsing table generation when you actually try this.
          //rewriteCPEAAPL,
          addActions,
          modifyFunctionDeclaration,
          modifyIfStatement,
          modifyConditionalExpression,
          modifyShortCircuitExpressions,
          modifyFunctionExpression,
          modifyDoWhileStatement,
          modifyWhileStatement,
          expandOptionals,
          modifyForStatement,
          modifyForInOfStatement,
          expandParameterizedRules,
          modifyBlock,
          translateRules,
          processLookaheads,
          addLiterals,
          transform,
          rewriteIdentifierName,
          convertTokenNames,
        ];
        break;
      default:
        unreachable();
    }
  }

  async transpile() {
    const esgrammar = await readAllText(Deno.stdin);
    let rules = readRules(esgrammar);
    for (const pass of this.passes_) {
      rules = pass(rules, this.options_);
    }
    return rules;
  }
}

// The following rule
//
//   NonTerminal ::
//     Term1 Term2
//     Term3 Term4
//
// will be converted into
//
//   {
//     name: 'NonTerminal',
//     values: [ 'Term1 Term2', 'Term3 Term4' ]
//   }
//
// The following rule
//
//   NonTerminal :: one of
//     Term1 Term2
//     Term3 Term4
//
// will be converted into
//
//   {
//     name: 'NonTerminal',
//     values: [ 'Term1', 'Term2', 'Term3', 'Term4' ]
//   }
//
function readRules(esgrammar) {
  const STATE_RULE = 0;
  const STATE_MEMBER = 1;

  let state = STATE_RULE;
  let name;
  let values;
  let oneOf;

  const rules = [];
  for (const line of esgrammar.split('\n')) {
    if (state === STATE_RULE) {
      if (line.trim().length === 0) {
        continue;
      }
      const parts = line.trim().split(/\s*[:]+\s*/u);
      name = parts.shift();
      if (parts[0] !== undefined && parts[0].startsWith('one of')) {
        oneOf = true;
      } else {
        oneOf = false;
      }
      values = [];
      state = STATE_MEMBER;
    } else if (state === STATE_MEMBER) {
      let trimed = line.trim();
      if (trimed.length === 0) {
        if (name === 'CodePoint') {
          // Special case: CodePoint
          rules.push({
            name,
            values: [
              'HexDigit',
              'HexDigit HexDigit',
              'HexDigit HexDigit HexDigit',
              'Hex4Digits',
              'HexDigit Hex4Digits',
              '`10` Hex4Digits',
              '`0` CodePoint',
            ],
          });
        } else if (name === 'NotCodePoint') {
          // Special case: NotCodePoint
          rules.push({
            name,
            values: [
              'NonZeroHexDigit NonZeroHexDigit Hex4Digits',
              'HexDigit NotCodePoint',
            ],
          });
        } else {
          rules.push({ name, values });
        }
        state = STATE_RULE;
        continue;
      }
      if (oneOf) {
        values = values.concat(trimed.split(/\s+/u));
      } else {
        // A production rule in the syntactic grammar ends with '#<name>' which
        // may be used for generating an anchor to it.  Remove '#<name>'.
        const pos = trimed.search(/#\w+$/);
        if (pos !== -1) {
          trimed = trimed.slice(0, pos).trim();
        }
        values.push(trimed);
      }
    }
  }

  return rules;
}

function addLexicalRules(rules) {
  // Additional rules.
  rules.push({
    name: 'NonZeroHexDigit',
    values: ['HexDigit but not `0`'],
  });
  rules.push({
    name: 'WhiteSpaceSequence',
    values: ['WhiteSpace WhiteSpaceSequence?'],
  });

  return rules;
}

// TODO: Read keywords from lexer/tokens.yaml.
const RESERVED_KEYWORDS = [
  'await',
  'break',
  'case',
  'catch',
  'class',
  'const',
  'continue',
  'debugger',
  'default',
  'delete',
  'do',
  'else',
  'enum',
  'export',
  'extends',
  'false',
  'finally',
  'for',
  'function',
  'if',
  'import',
  'in',
  'instanceof',
  'new',
  'null',
  'return',
  'super',
  'switch',
  'this',
  'throw',
  'true',
  'try',
  'typeof',
  'var',
  'void',
  'while',
  'with',
  'yield',
];

// TODO: Read keywords from lexer/tokens.yaml.
const ADDITIONAL_KEYWORDS = [
  // KeywordInStrictMode
  'let',
  'static',
  'implements',
  'interface',
  'package',
  'private',
  'protected',
  'public',
  // UnreservedKeyword
  'as',
  'async',
  'from',
  'get',
  'meta',
  'of',
  'set',
  'target',
];

function rewriteReservedWord(rules) {
  log.debug('Rewriting ReservedWord...');
  const rule = rules.find((rule) => rule.name === 'ReservedWord');
  assert(rule !== undefined);
  const values = rule.values;
  rule.values = [];
  for (const reserved of values) {
    rule.values.push(reserved.slice(1, -1).toUpperCase());
    rules.push({
      name: reserved.slice(1, -1).toUpperCase(),
      values: [reserved],
    });
  }
  for (const word of ADDITIONAL_KEYWORDS) {
    rule.values.push(word.toUpperCase());
    rules.push({
      name: word.toUpperCase(),
      values: [`\`${word}\``],
    });
  }
  return rules;
}

const PUNCTUATORS = {
  '?.': 'OPTIONAL_CHAINING',
  '{': 'LBRACE',
  '}': 'RBRACE',
  '[': 'LBRACK',
  ']': 'RBRACK',
  '(': 'LPAREN',
  ')': 'RPAREN',
  '.': 'DOT',
  '...': 'ELLIPSIS',
  ';': 'SEMICOLON',
  ',': 'COMMA',
  '<': 'LT',
  '>': 'GT',
  '<=': 'LTE',
  '>=': 'GTE',
  '==': 'EQ',
  '!=': 'NE',
  '===': 'EQ_STRICT',
  '!==': 'NE_STRICT',
  '+': 'ADD',
  '-': 'SUB',
  '*': 'MUL',
  '/': 'DIV',
  '%': 'MOD',
  '**': 'EXP',
  '++': 'INC',
  '--': 'DEC',
  '<<': 'SHL',
  '>>': 'SAR',
  '>>>': 'SHR',
  '&': 'BIT_AND',
  '|': 'BIT_OR',
  '^': 'BIT_XOR',
  '!': 'NOT',
  '~': 'BIT_NOT',
  '&&': 'AND',
  '||': 'OR',
  '??': 'NULLISH',
  '?': 'CONDITIONAL',
  ':': 'COLON',
  '=': 'ASSIGN',
  '+=': 'ADD_ASSIGN',
  '-=': 'SUB_ASSIGN',
  '*=': 'MUL_ASSIGN',
  '/=': 'DIV_ASSIGN',
  '%=': 'MOD_ASSIGN',
  '**=': 'EXP_ASSIGN',
  '<<=': 'SHL_ASSIGN',
  '>>=': 'SAR_ASSIGN',
  '>>>=': 'SHR_ASSIGN',
  '&=': 'BIT_AND_ASSIGN',
  '|=': 'BIT_OR_ASSIGN',
  '^=': 'BIT_XOR_ASSIGN',
  '&&=': 'AND_ASSIGN',
  '||=': 'OR_ASSIGN',
  '??=': 'NULLISH_ASSIGN',
  '=>': 'ARROW',
};

function rewritePunctuator(rules) {
  log.debug('Rewriting Punctuators...');
  const optionalChaining = rules.find((rule) => rule.name === 'OptionalChainingPunctuator');
  rules.push({
    name: 'OPTIONAL_CHAINING',
    values: optionalChaining.values,
  });
  optionalChaining.values = ['OPTIONAL_CHAINING'];

  const otherPunctuator = rules.find((rule) => rule.name === 'OtherPunctuator');
  otherPunctuator.values.forEach((value) => {
    rules.push({
      name: PUNCTUATORS[value.slice(1, -1)],
      values: [value],
    });
  });
  otherPunctuator.values = otherPunctuator.values.map((value) => {
    return PUNCTUATORS[value.slice(1, -1)];
  });

  const divPunctuator = rules.find((rule) => rule.name === 'DivPunctuator');
  divPunctuator.values.forEach((value) => {
    rules.push({
      name: PUNCTUATORS[value.slice(1, -1)],
      values: [value],
    });
  });
  divPunctuator.values = divPunctuator.values.map((value) => {
    return PUNCTUATORS[value.slice(1, -1)];
  });

  const rightBracePunctuator = rules.find((rule) => rule.name === 'RightBracePunctuator');
  rightBracePunctuator.values.forEach((value) => {
    rules.push({
      name: PUNCTUATORS[value.slice(1, -1)],
      values: [value],
    });
  });
  rightBracePunctuator.values = rightBracePunctuator.values.map((value) => {
    return PUNCTUATORS[value.slice(1, -1)];
  });

  return rules;
}

function rewriteIdentifierRule(rules) {
  log.debug('Rewriting Identifier rule...');
  const rule = rules.find((rule) => rule.name === 'Identifier');
  assert(rule !== undefined);
  assert(rule.values[0] === 'IdentifierName but not ReservedWord');
  // A generated lexer recognizes the reserved words as separate tokens.
  rule.values = ['IdentifierNameButNotReservedWord'];
  return rules;
}

// Unkine other production rules for binary operators such as `+`, `MultiplicativeExpression` is
// defined by using `MultiplicativeOperator`.  This causes a bothersome complication in the
// semantic analysis.  This function replaces `MultiplicativeOperaor` in the production rule with
// actual operators.
function expandMultiplicativeOperator(rules) {
  log.debug('Expanding MultiplicativeOperator...');
  const rule = rules.find((rule) => rule.name === 'MultiplicativeExpression[Yield, Await]');
  assert(rule !== undefined);
  assert(rule.values.length === 2);
  const value = rule.values.pop();
  assert(
    value ===
      'MultiplicativeExpression[?Yield, ?Await] MultiplicativeOperator ExponentiationExpression[?Yield, ?Await]',
  );
  const multiplicativeOperatorRule = rules.find((rule) => rule.name === 'MultiplicativeOperator');
  assert(multiplicativeOperatorRule !== undefined);
  for (const op of multiplicativeOperatorRule.values) {
    rule.values.push(value.replace('MultiplicativeOperator', op));
  }
  return rules;
}

// For the same reason as `MultiplicativeOperator`, `LetOrConst` terms in production rules are
// expanded.
function expandLetOrConst(rules) {
  log.debug('Expanding LetOrConst...');
  for (const rule of rules) {
    const values = [];
    for (const value of rule.values) {
      if (value.includes('LetOrConst')) {
        values.push(value.replace('LetOrConst', 'LET'));
        values.push(value.replace('LetOrConst', 'CONST'));
      } else {
        values.push(value);
      }
    }
    rule.values = values;
  }
  return rules;
}

function rewriteCPEAAPL(rules) {
  log.debug('Rewriting CPEAAPL...');

  let rule;

  // Replace CPEAAPL in PrimaryExpression with ParenthesizedExpression.
  rule = rules.find((rule) => rule.name === 'PrimaryExpression[Yield, Await]');
  assert(rule !== undefined);
  for (let i = 0; i < rule.values.length; ++i) {
    if (rule.values[i].startsWith('CoverParenthesizedExpressionAndArrowParameterList')) {
      rule.values[i] = 'ParenthesizedExpression[?Yield, ?Await]';
      break;
    }
  }

  // Replace CPEAAPL in ArrowParameters with ArrowFormalParameters.
  rule = rules.find((rule) => rule.name === 'ArrowParameters[Yield, Await]');
  assert(rule !== undefined);
  for (let i = 0; i < rule.values.length; ++i) {
    if (rule.values[i].startsWith('CoverParenthesizedExpressionAndArrowParameterList')) {
      rule.values[i] = 'ArrowFormalParameters[?Yield, ?Await]';
      break;
    }
  }

  return rules;
}

function addActions(rules) {
  log.debug('Adding production rules for semantic actions...');

  const ACTIONS = [
    '_FUNCTION_CONTEXT_',
    '_FUNCTION_SIGNATURE_',
    '_ELSE_BLOCK_',
    '_THEN_BLOCK_',
    '_BLOCK_SCOPE_',
    '_FALSY_SHORT_CIRCUIT_',
    '_TRUTHY_SHORT_CIRCUIT_',
    '_NULLISH_SHORT_CIRCUIT_',
    '_FALSY_SHORT_CIRCUIT_ASSIGNMENT_',
    '_TRUTHY_SHORT_CIRCUIT_ASSIGNMENT_',
    '_NULLISH_SHORT_CIRCUIT_ASSIGNMENT_',
    '_LOOP_START_',
    '_LOOP_INIT_',
    '_LOOP_TEST_',
    '_LOOP_NEXT_',
  ];

  for (const action of ACTIONS) {
    rules.push({
      name: action,
      values: ['[empty]'],
    });
  }

  return rules;
}

function modifyFunctionDeclaration(rules) {
  // The action will be inserted before the token.
  const TARGETS = [
    {
      token: '`(`',
      action: '_FUNCTION_CONTEXT_',
    },
    {
      token: '`{`',
      action: '_FUNCTION_SIGNATURE_',
    },
  ];

  log.debug('Modifying FunctionDeclaration...');

  const rule = rules.find((rule) => rule.name === 'FunctionDeclaration[Yield, Await, Default]');
  assert(rule !== undefined);

  for (let i = 0; i < rule.values.length; ++i) {
    for (const target of TARGETS) {
      const [head, tail] = rule.values[i].split(target.token);
      rule.values[i] = [head, target.action, target.token, tail].join(' ');
    }
  }

  return rules;
}

function modifyIfStatement(rules) {
  log.debug('Modifying IfStatement...');

  let rule;

  rule = rules.find((rule) => rule.name === 'IfStatement[Yield, Await, Return]');
  assert(rule !== undefined);
  assert(rule.values.length === 2);

  rule.values[0] = rule
    .values[0]
    .replace('`)` Statement[', '`)` _THEN_BLOCK_ Statement[')
    .replace('`else` Statement[', '`else` _ELSE_BLOCK_ Statement[');

  rule.values[1] = rule
    .values[1]
    .replace('`)` Statement[', '`)` _THEN_BLOCK_ Statement[');

  return rules;
}

function modifyConditionalExpression(rules) {
  log.debug('Modifying ConditionalExpression...');

  let rule;

  rule = rules.find((rule) => rule.name === 'ConditionalExpression[In, Yield, Await]');
  assert(rule !== undefined);
  assert(rule.values.length === 2);
  const [cond, thenBlock, elseBlock] = rule
    .values[1]
    .split(/`\?`|`\:`/)
    .map((term) => term.trim());
  rule.values[1] = [
    cond,
    '`?`',
    '_THEN_BLOCK_',
    thenBlock,
    '`:`',
    '_ELSE_BLOCK_',
    elseBlock,
  ].join(' ');

  return rules;
}

function modifyShortCircuitExpressions(rules) {
  const TARGETS = [
    {
      rule: 'LogicalANDExpression[In, Yield, Await]',
      op: '`&&`',
      action: '_FALSY_SHORT_CIRCUIT_',
    },
    {
      rule: 'LogicalORExpression[In, Yield, Await]',
      op: '`||`',
      action: '_TRUTHY_SHORT_CIRCUIT_',
    },
    {
      rule: 'CoalesceExpression[In, Yield, Await]',
      op: '`??`',
      action: '_NULLISH_SHORT_CIRCUIT_',
    },
    {
      rule: 'AssignmentExpression[In, Yield, Await]',
      op: '`&&=`',
      action: '_FALSY_SHORT_CIRCUIT_ASSIGNMENT_',
    },
    {
      rule: 'AssignmentExpression[In, Yield, Await]',
      op: '`||=`',
      action: '_TRUTHY_SHORT_CIRCUIT_ASSIGNMENT_',
    },
    {
      rule: 'AssignmentExpression[In, Yield, Await]',
      op: '`??=`',
      action: '_NULLISH_SHORT_CIRCUIT_ASSIGNMENT_',
    },
  ];

  for (const target of TARGETS) {
    log.debug(`Modifying ${target.rule}...`);
    const rule = rules.find((rule) => rule.name === target.rule);
    assert(rule !== undefined);
    const index = rule.values.findIndex((production) => production.includes(target.op));
    assert(index !== -1);
    const [lhs, rhs] = rule.values[index].split(target.op).map((term) => term.trim());
    // Insert target.action for the short-circuit evaluation of the LHS.
    rule.values[index] = [lhs, target.op, target.action, rhs].join(' ');
  }

  return rules;
}

function modifyFunctionExpression(rules) {
  // The action will be inserted before the token.
  const TARGETS = [
    {
      token: '`(`',
      action: '_FUNCTION_CONTEXT_',
    },
    {
      token: '`{`',
      action: '_FUNCTION_SIGNATURE_',
    },
  ];

  log.debug('Modifying FunctionExpression...');

  const rule = rules.find((rule) => rule.name === 'FunctionExpression');
  assert(rule !== undefined);

  for (let i = 0; i < rule.values.length; ++i) {
    for (const target of TARGETS) {
      const [head, tail] = rule.values[i].split(target.token);
      rule.values[i] = [head, target.action, target.token, tail].join(' ');
    }
  }

  return rules;
}

// CAUTION: You MUST update `isAutoSemicolonDoWhile()` in parser/lalr.js when you change the
// production rule of `DoWhileStatement`.
function modifyDoWhileStatement(rules) {
  const TARGETS = [
    {
      term: '`do`',
      action: '_LOOP_START_',
      insertBefore: false,
    },
    {
      term: '`)`',
      action: '_LOOP_TEST_',
      insertBefore: false,
    },
  ];

  log.debug('Modifying WhileStatement...');

  const rule = rules.find((rule) => rule.name === 'DoWhileStatement[Yield, Await, Return]');
  assert(rule !== undefined);
  assert(rule.values.length === 1);

  rule.values[0] = modifyTargetsInProduction(rule.values[0], TARGETS);

  return rules;
}

function modifyWhileStatement(rules) {
  const TARGETS = [
    {
      term: '`(`',
      action: '_LOOP_START_',
      insertBefore: true,
    },
    {
      term: '`)`',
      action: '_LOOP_TEST_',
      insertBefore: false,
    },
  ];

  log.debug('Modifying WhileStatement...');

  const rule = rules.find((rule) => rule.name === 'WhileStatement[Yield, Await, Return]');
  assert(rule !== undefined);
  assert(rule.values.length === 1);

  rule.values[0] = modifyTargetsInProduction(rule.values[0], TARGETS);

  return rules;
}

function modifyTargetsInProduction(production, targets) {
  for (const target of targets) {
    production = modifyTargetInProduction(production, target);
  }
  return production;
}

function modifyTargetInProduction(production, target) {
  const [head, tail] = production.split(target.term).map((term) => term.trim());
  if (tail === undefined) {
    return production;
  }
  let terms;
  if (target.insertBefore) {
    terms = [head, target.action, target.term, tail];
  } else {
    terms = [head, target.term, target.action, tail];
  }
  return terms.filter((term) => term !== '').join(' ');
}

function expandOptionals(rules) {
  log.debug('Expanding optionals...');
  const expanded = [];
  for (const rule of rules) {
    const values = [];
    let hasOptionals = false;
    for (const value of rule.values) {
      // A parameter list contains spaces like below:
      //
      //  AssignmentRestProperty[?Yield, ?Await]?
      //
      const parts = value.split(/(?<!,)\s+/u);
      let patterns = [[]];
      for (let part of parts) {
        if (part.endsWith('?')) {
          hasOptionals = true;
          const clone = patterns.map((pattern) => Array.from(pattern));
          part = part.slice(0, -1);
          clone.forEach((pattern) => pattern.push(part));
          patterns = patterns.concat(clone);
        } else {
          patterns.forEach((pattern) => pattern.push(part));
        }
      }
      patterns.forEach((pattern) => {
        if (pattern.length === 0) {
          values.push('[empty]');
        } else {
          values.push(pattern.join(' '));
        }
      });
    }
    if (hasOptionals) {
      log.debug(`  ${rule.name}`);
    }
    expanded.push({
      name: rule.name,
      values,
    });
  }
  return expanded;
}

function modifyForStatement(rules) {
  // DO NOT CHANGE THE ORDER OF ELEMENTS IN THE TARGETS.
  const TARGETS = [
    {
      term: '`;` Expression[+In, ?Yield, ?Await] `)`',
      action: '_LOOP_NEXT_',
      insertBefore: false,
    },
    {
      term: '`;` Expression[+In, ?Yield, ?Await] `;`',
      action: '_LOOP_TEST_',
      insertBefore: false,
    },
    {
      term: 'LexicalDeclaration[~In, ?Yield, ?Await] Expression[+In, ?Yield, ?Await] `;`',
      action: '_LOOP_TEST_',
      insertBefore: false,
    },
    {
      term: '[lookahead != `let` `[`] Expression[~In, ?Yield, ?Await] `;`',
      action: '_LOOP_INIT_',
      insertBefore: false,
    },
    {
      term: '`var` VariableDeclarationList[~In, ?Yield, ?Await] `;`',
      action: '_LOOP_INIT_',
      insertBefore: false,
    },
    {
      term: 'LexicalDeclaration[~In, ?Yield, ?Await]',
      action: '_LOOP_INIT_',
      insertBefore: false,
    },
    // NOTE: Inserting _BLOCK_SCOPE_ before LexicalDeclaration[~In, ?Yield, ?Await] in order to
    // create a new lexical scope causes shift/reduce conflicts.  Other similar methods also cause
    // conflicts.  Eventually, we decided to always create a new lexical scope in the action for
    // _LOOP_START_.  See also comments in `jsruntime::semantics::Analyzer::handle_loop_start()`.
    {
      term: '`(`',
      action: '_LOOP_START_',
      insertBefore: true,
    },
  ];

  log.debug('Modifying ForStatement...');

  const rule = rules.find((rule) => rule.name === 'ForStatement[Yield, Await, Return]');
  assert(rule !== undefined);

  for (let i = 0; i < rule.values.length; ++i) {
    rule.values[i] = modifyTargetsInProduction(rule.values[i], TARGETS);
  }

  return rules;
}

function modifyForInOfStatement(rules) {
  // TODO: Add targets.
  // At this point, _LOOP_START_ is inserted in order to avoid shift/reduce conflicts.
  const TARGETS = [
    {
      term: '`(`',
      action: '_LOOP_START_',
      insertBefore: true,
    },
  ];

  log.debug('Modifying ForInOfStatement...');

  const rule = rules.find((rule) => rule.name === 'ForInOfStatement[Yield, Await, Return]');
  assert(rule !== undefined);

  for (let i = 0; i < rule.values.length; ++i) {
    rule.values[i] = modifyTargetsInProduction(rule.values[i], TARGETS);
  }

  return rules;
}

function expandParameterizedRules(rules) {
  log.debug('Expanding parameterized rules...');
  const expanded = [];

  for (const rule of rules) {
    if (rule.name.endsWith(']')) {
      log.debug(`  ${rule.name}`);
      const [name, paramList] = rule.name.split(/[\[\]]/u);
      const params = paramList.split(', ');
      const combinations = buildParameterCombinations(params);
      for (const combination of combinations) {
        log.debug(`    combination:  ${JSON.stringify(combination)}`);
        const values = [];
        for (let value of rule.values) {
          let valueToBeExpanded;
          if (value.startsWith('[+')) {
            const pos = value.indexOf(']');
            assert(pos !== -1);
            const param = value.slice(2, pos);
            const remaining = value.slice(pos + 1).trim();
            if (!combination.includes(param)) {
              log.debug(`      ${value} ->`);
              continue;
            }
            valueToBeExpanded = remaining;
          } else if (value.startsWith('[~')) {
            const pos = value.indexOf(']');
            assert(pos !== -1);
            const param = value.slice(2, pos);
            const remaining = value.slice(pos + 1).trim();
            if (combination.includes(param)) {
              log.debug(`      ${value} ->`);
              continue;
            }
            valueToBeExpanded = remaining;
          } else {
            valueToBeExpanded = value;
          }
          const expandedValue = expandParameterizedValue(valueToBeExpanded, combination);
          log.debug(`      ${value}`);
          log.debug(`       -> ${expandedValue}`);
          values.push(expandedValue);
        }
        if (values.length > 0) {
          expanded.push({
            name: expandRuleName(name, combination),
            values,
          });
        }
      }
    } else {
      expanded.push({
        name: rule.name,
        values: rule.values.map(expandParameterizedValue),
      });
    }
  }
  return expanded;
}

function buildParameterCombinations(params) {
  assert(params.length > 0);
  const param = params[0];
  if (params.length === 1) {
    return [[], [param]];
  }
  const remaining = params.slice(1);
  const combinations = buildParameterCombinations(remaining);
  return combinations.concat(combinations.map((params) => [param].concat(params)));
}

function expandRuleName(name, combination) {
  return [name, ...combination].join('_');
}

function expandParameterizedValue(value, combination) {
  let expanded = '';
  for (;;) {
    let pos = value.search(/\[[?+~]/);
    if (pos === -1) {
      expanded = expanded + value;
      break;
    }
    expanded = expanded + value.slice(0, pos);
    value = value.slice(pos);
    pos = value.indexOf(']');
    assert(pos !== -1);
    const patterns = value
      .slice(1, pos) // remove '[' and ']'
      .split(', ');
    const suffix = expandSuffixPatterns(patterns, combination);
    if (suffix.length > 0) {
      expanded = expanded + '_' + suffix;
    }
    value = value.slice(pos + 1); // remove '[...]'
  }
  return expanded;
}

function expandSuffixPatterns(patterns, combination) {
  let params = [];
  for (const pattern of patterns) {
    if (pattern.startsWith('~')) {
      continue;
    }
    if (pattern.startsWith('+')) {
      params.push(pattern.slice(1));
      continue;
    }
    if (pattern.startsWith('?')) {
      const param = pattern.slice(1);
      if (combination.includes(param)) {
        params.push(param);
      }
    }
  }
  return params.join('_');
}

function modifyBlock(rules) {
  log.debug('Modifying Block...');

  const blockRules = rules.filter((rule) => {
    return rule.name === 'Block' || rule.name.startsWith('Block_');
  });

  let rule;

  for (const rule of blockRules) {
    assert(rule.values.length === 2);
    rule.values[1] = rule
      .values[1]
      .replace('`{` Statement', '`{` _BLOCK_SCOPE_ Statement');
  }

  return rules;
}

function translateRules(rules, options) {
  const grammar = [];
  for (const rule of rules) {
    log.debug(`Translating ${rule.name}...`);
    grammar.push({
      name: rule.name,
      productions: rule.values.map((value) => translateProduction(value, options)),
    });
  }
  return grammar;
}

// A production is an array of terms.
function translateProduction(value, options) {
  log.debug(`  ${value}`);

  // Special case: ID_Start
  if (value === '> any Unicode code point with the Unicode property “ID_Start”') {
    // NOTE: Non-ASCII characters included in ID_Start will be handled when the first lookup fails.
    // See `recognize()` in dfa/dfa.rs.hbs for details.
    return [{
      type: 'unicode-set',
      data: [
        { type: 'span', data: ['a', 'z'] },
        { type: 'span', data: ['A', 'Z'] },
        { type: 'char', data: '$' },
        { type: 'char', data: '_' },
      ],
    }];
  }

  // Special case: ID_Continue
  if (value === '> any Unicode code point with the Unicode property “ID_Continue”') {
    // NOTE: Non-ASCII characters included in ID_Continue will be handled when the first lookup
    // fails.  See `recognize()` in dfa/dfa.rs.hbs for details.
    return [{
      type: 'unicode-set',
      data: [
        { type: 'span', data: ['0', '9'] },
        { type: 'span', data: ['a', 'z'] },
        { type: 'span', data: ['A', 'Z'] },
        { type: 'char', data: '$' },
        { type: 'char', data: '_' },
      ],
    }];
  }

  // Special case: X but not one of ...
  if (value.includes('but not one of')) {
    const [base, ...excludes] = value.replace('but not one of', '').replaceAll(' or', '').split(
      /\s+/u,
    );
    return [{
      type: 'unicode-set',
      data: [
        { type: 'non-terminal', data: base },
        ...excludes.map((exclude) => {
          if (exclude.startsWith('`')) {
            return { type: 'exclude', data: exclude.slice(1, -1) };
          }
          return { type: 'exclude', data: exclude };
        }),
      ],
    }];
  }

  // Special case: X but not ...
  if (value.includes('but not')) {
    const [base, ...excludes] = value.replace('but not', '').split(/\s+/u);
    return [{
      type: 'unicode-set',
      data: [
        { type: 'non-terminal', data: base },
        ...excludes.map((exclude) => {
          if (exclude.startsWith('`')) {
            return { type: 'exclude', data: exclude.slice(1, -1) };
          }
          return { type: 'exclude', data: exclude };
        }),
      ],
    }];
  }

  // Special case: [no LineTerminator here]
  value = value.replaceAll('[no LineTerminator here]', '[no-line-terminator]');

  let production = [];
  const terms = value.split(/\s+/u);
  while (terms.length > 0) {
    let term = terms.shift();
    if (term.startsWith('`')) {
      const str = term.slice(1, -1);
      if (options.grammarType === 'lexical') {
        // We assume that `str` contains only ASCII characters.
        if (str.length === 1) {
          production.push({
            type: 'unicode-set',
            data: [{ type: 'char', data: str }],
          });
        } else {
          for (const ch of str) {
            production.push({
              type: 'unicode-set',
              data: [{ type: 'char', data: ch }],
            });
          }
        }
      } else {
        assertEquals(options.grammarType, 'syntactic');
        let token = PUNCTUATORS[str];
        if (token === undefined) {
          token = str.toUpperCase();
        }
        production.push({ type: 'token', data: token });
      }
    } else if (term.startsWith('<')) {
      assertEquals(options.grammarType, 'lexical');
      // TODO: Use the Unicode escape sequence and remove the 'built-on' type.
      production.push({
        type: 'unicode-set',
        data: [{ type: 'built-in', data: term.slice(1, -1) }],
      });
    } else if (term === '[lookahead') {
      production = production.concat(translateLookahead(terms, options));
    } else if (term === '[no-line-terminator]') {
      production.push({ type: 'disallow', data: 'LineTerminatorSequence' });
    } else if (term === '[empty]') {
      production.push({ type: 'empty' });
    } else if (options.tokens?.includes(term)) {
      production.push({ type: 'token', data: term });
    } else {
      production.push({ type: 'non-terminal', data: term });
    }
  }

  return production;
}

function translateLookahead(terms, options) {
  let op = terms.shift();
  let target = terms.shift();
  let values;
  if (target === '{') {
    values = [];
    let seq = [];
    target = terms.shift();
    for (;;) {
      if (target === '}]') {
        if (seq.length > 0) {
          values.push(seq);
        }
        break;
      }
      if (target.endsWith(',')) {
        seq.push(target.slice(0, -1));
        values.push(seq);
        seq = [];
      } else {
        seq.push(target);
      }
      target = terms.shift();
    }
  } else {
    const seq = [];
    for (;;) {
      if (target.endsWith(']')) {
        seq.push(target.slice(0, -1)); // remove the last ']'
        break;
      } else {
        seq.push(target);
      }
      target = terms.shift();
    }
    values = [seq];
  }
  switch (op) {
    case '=':
      return translateLookaheadSet(values, false, options);
    case '!=':
      return translateLookaheadSet(values, true, /* negate */ options);
    case '\u2208':
      return translateLookaheadSet(values, false, options);
    case '\u2209':
      return translateLookaheadSet(values, true, /* negate */ options);
    default:
      log.error(`translateLookahead: Unknown op: U+${op.codePointAt(0).toString(16)}`);
      Deno.exit(1);
  }
}

function translateLookaheadSet(values, negate, options) {
  const patterns = [];
  for (let seq of values) {
    let pattern = seq.map((value) => {
      if (value.startsWith('`')) {
        if (options.grammarType === 'lexical') {
          return { type: 'char', data: value.slice(1, -1) };
        } else {
          assertEquals(options.grammarType, 'syntactic');
          const str = value.slice(1, -1);
          let token = PUNCTUATORS[str];
          if (token === undefined) {
            token = str.toUpperCase();
          }
          return { type: 'token', data: token };
        }
      } else if (value.startsWith('<')) {
        assertEquals(options.grammarType, 'lexical');
        return { type: 'built-in', data: value.slice(1, -1) };
      } else if (value === '[no-line-terminator]') {
        assertEquals(options.grammarType, 'syntactic');
        return { type: 'disallow', data: 'LineTerminatorSequence' };
      } else {
        assertEquals(options.grammarType, 'lexical');
        return { type: 'non-terminal', data: value };
      }
    });
    // In a lexical grammar, a pattern is a single term.
    // In a syntactic grammar, a pattern is a sequence of terms.
    if (options.grammarType === 'lexical') {
      assertEquals(pattern.length, 1);
      pattern = pattern[0];
    } else {
      assertEquals(options.grammarType, 'syntactic');
    }
    patterns.push(pattern);
  }
  return {
    type: 'lookahead',
    data: { patterns, negate },
  };
}

function addSourceCharacter(rules) {
  log.debug(`Adding SourceCharacter...`);
  return [
    { name: 'SourceCharacter', productions: [[{ type: 'any' }]] },
    ...rules,
  ];
}

function mergeUnicodeSets(rules) {
  for (const rule of rules) {
    const doMerge = rule.productions.every((production) => {
      return production.length === 1 && production[0].type === 'unicode-set';
    });
    if (doMerge) {
      log.debug(`Merging unicode sets in ${rule.name}...`);
      rule.productions = [[{
        type: 'unicode-set',
        data: rule.productions.reduce((data, production) => {
          return data.concat(production[0].data);
        }, []),
      }]];
    }
  }
  return rules;
}

function processLookaheads(rules) {
  const context = {
    ruleMap: {},
    newRules: [],
  };
  for (const rule of rules) {
    context.ruleMap[rule.name] = rule;
  }
  for (const rule of rules) {
    rule.productions.forEach((production, index) => {
      return processLookaheadsInProduction(context, rule.name, production, index);
    });
  }
  return rules;
}

function processLookaheadsInProduction(context, name, production, index) {
  for (const term of production) {
    switch (term.type) {
      case 'lookahead':
        log.debug(`Processing lookaheads in ${name}...`);
        // A pattern is a sequence of tokens.
        const data = term.data.patterns.map((pattern) => {
          return pattern.map((term) => {
            switch (term.type) {
              case 'token':
                return term.data;
              case 'disallow':
                return `(!${term.data})`;
              default:
                unreachable();
            }
          });
        });
        if (term.data.negate) {
          term.data = { type: 'exclude', data };
        } else {
          term.data = { type: 'include', data };
        }
        break;
    }
  }
}

function addLiterals(rules) {
  rules.push({
    name: 'NullLiteral',
    productions: [
      [{ type: 'token', data: 'NULL' }],
    ],
  });
  rules.push({
    name: 'BooleanLiteral',
    productions: [
      [{ type: 'token', data: 'TRUE' }],
      [{ type: 'token', data: 'FALSE' }],
    ],
  });
  return rules;
}

function printYaml(rules) {
  console.log(HEADER);
  console.log('');
  console.log(yaml.stringify(rules).trim());
}

if (import.meta.main) {
  const { options, args } = await parseCommand({
    doc: DOC,
    conv: async (name, value) => {
      switch (name) {
        case '--tokens':
          if (value) {
            return JSON.parse(await Deno.readTextFile(value));
          }
          return value;
        default:
          return value;
      }
    },
  });
  Deno.exit(await run(options));
}

// tests

setup(PROGNAME, 'DEBUG');

Deno.test('expandRuleName', () => {
  assertEquals(expandRuleName('R', []), 'R');
  assertEquals(expandRuleName('R', ['A']), 'R_A');
  assertEquals(expandRuleName('R', ['A', 'B']), 'R_A_B');
});

Deno.test('expandSuffixPatterns', () => {
  assertEquals(expandSuffixPatterns(['+A', '~B', '?C'], []), 'A');
  assertEquals(expandSuffixPatterns(['+A', '~B', '?C'], ['C']), 'A_C');
});

Deno.test('expandParameterizedValue', () => {
  const VALUE = 'R1[+A] R2[~B] R3[?C] R4[+A, ~B, ?C] R5';
  assertEquals(
    expandParameterizedValue(VALUE, []),
    'R1_A R2 R3 R4_A R5',
  );
  assertEquals(
    expandParameterizedValue(VALUE, ['C']),
    'R1_A R2 R3_C R4_A_C R5',
  );
});
