'use strict';

import * as log from '@std/log';
import * as path from '@std/path';
import * as yaml from '@std/yaml';
import * as changeCase from 'change-case';
import { parseCommand } from '../../../../tools/lib/cli.js';
import { setup } from '../../../../tools/lib/log.js';

const PROGNAME = path.basename(path.fromFileUrl(import.meta.url));

const DOC = `
Usage:
  ${PROGNAME} <builtins.yaml>
  ${PROGNAME} -h | --help

Options:
  -d, --debug
    Enable debug logs.
`;

const { options, args } = await parseCommand({
  doc: DOC,
});

if (options.debug) {
  setup(PROGNAME, 'DEBUG');
} else {
  setup(PROGNAME, 'INFO');
}

Deno.exit(await main(args, options));

async function main(args, options) {
  log.debug(`Loading ${args.builtinsYaml}...`);
  const builtinsYaml = await Deno.readTextFile(args.builtinsYaml);
  const builtins = yaml
    .parse(builtinsYaml)
    .map((name) => {
      const codeUnits = [];
      for (let i = 0; i < name.length; ++i) {
        codeUnits.push(name.charCodeAt(i));
      }
      return {
        name,
        rustName: makeRustName(name),
        codeUnits: makeCodeUnits(name),
        hidden: name.startsWith('##'),
      };
    });
  console.log(JSON.stringify(builtins));
}

function makeRustName(name) {
  if (name.startsWith('##')) {
    return `HIDDEN_${changeCase.constantCase(name)}`;
  }
  // Use `upper` instead of `constantCase` so that 'NaN' is converted into 'NAN'.
  return name.toUpperCase();
}

function makeCodeUnits(name) {
  const codeUnits = [];
  for (let i = 0; i < name.length; ++i) {
    codeUnits.push(name.charCodeAt(i));
  }
  return codeUnits;
}
