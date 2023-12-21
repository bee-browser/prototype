// DO NOT EDIT THIS FILE BY HAND.
//
// This file was automagically generated by:
// codegen.js --input-stdin --no-escape lalr/auto_semicolon.rs.hbs

use phf::phf_set;
use phf::Set;

pub const DISALLOWED: Set<u16> = phf_set! {
    // State(43)
    //   [EmptyStatement -> SEMICOLON .]*
    43u16,
    // State(680)
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) SEMICOLON . SEMICOLON RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) SEMICOLON . SEMICOLON Expression_In RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) SEMICOLON . Expression_In SEMICOLON RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) SEMICOLON . Expression_In SEMICOLON Expression_In RPAREN Statement]*
    680u16,
    // State(907)
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) SEMICOLON . SEMICOLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) SEMICOLON . SEMICOLON Expression_In_Await RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) SEMICOLON . Expression_In_Await SEMICOLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) SEMICOLON . Expression_In_Await SEMICOLON Expression_In_Await RPAREN Statement_Await]*
    907u16,
    // State(1106)
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) SEMICOLON SEMICOLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) SEMICOLON SEMICOLON . Expression_In RPAREN Statement]*
    1106u16,
    // State(1124)
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) Expression SEMICOLON . SEMICOLON RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) Expression SEMICOLON . SEMICOLON Expression_In RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) Expression SEMICOLON . Expression_In SEMICOLON RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) Expression SEMICOLON . Expression_In SEMICOLON Expression_In RPAREN Statement]*
    1124u16,
    // State(1140)
    //   [ForStatement -> FOR LPAREN LexicalDeclaration SEMICOLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN LexicalDeclaration SEMICOLON . Expression_In RPAREN Statement]*
    1140u16,
    // State(1331)
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) SEMICOLON SEMICOLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) SEMICOLON SEMICOLON . Expression_In_Await RPAREN Statement_Await]*
    1331u16,
    // State(1349)
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON . SEMICOLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON . SEMICOLON Expression_In_Await RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON . Expression_In_Await SEMICOLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON . Expression_In_Await SEMICOLON Expression_In_Await RPAREN Statement_Await]*
    1349u16,
    // State(1365)
    //   [ForStatement_Await -> FOR LPAREN LexicalDeclaration_Await SEMICOLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN LexicalDeclaration_Await SEMICOLON . Expression_In_Await RPAREN Statement_Await]*
    1365u16,
    // State(1493)
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) SEMICOLON Expression_In SEMICOLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) SEMICOLON Expression_In SEMICOLON . Expression_In RPAREN Statement]*
    1493u16,
    // State(1500)
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMICOLON . SEMICOLON RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMICOLON . SEMICOLON Expression_In RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMICOLON . Expression_In SEMICOLON RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMICOLON . Expression_In SEMICOLON Expression_In RPAREN Statement]*
    1500u16,
    // State(1520)
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) Expression SEMICOLON SEMICOLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) Expression SEMICOLON SEMICOLON . Expression_In RPAREN Statement]*
    1520u16,
    // State(1537)
    //   [ForStatement -> FOR LPAREN LexicalDeclaration Expression_In SEMICOLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN LexicalDeclaration Expression_In SEMICOLON . Expression_In RPAREN Statement]*
    1537u16,
    // State(1652)
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) SEMICOLON Expression_In_Await SEMICOLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) SEMICOLON Expression_In_Await SEMICOLON . Expression_In_Await RPAREN Statement_Await]*
    1652u16,
    // State(1659)
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMICOLON . SEMICOLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMICOLON . SEMICOLON Expression_In_Await RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMICOLON . Expression_In_Await SEMICOLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMICOLON . Expression_In_Await SEMICOLON Expression_In_Await RPAREN Statement_Await]*
    1659u16,
    // State(1671)
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON SEMICOLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON SEMICOLON . Expression_In_Await RPAREN Statement_Await]*
    1671u16,
    // State(1688)
    //   [ForStatement_Await -> FOR LPAREN LexicalDeclaration_Await Expression_In_Await SEMICOLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN LexicalDeclaration_Await Expression_In_Await SEMICOLON . Expression_In_Await RPAREN Statement_Await]*
    1688u16,
    // State(1811)
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMICOLON SEMICOLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMICOLON SEMICOLON . Expression_In RPAREN Statement]*
    1811u16,
    // State(1819)
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) Expression SEMICOLON Expression_In SEMICOLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN (?![LET LBRACK]) Expression SEMICOLON Expression_In SEMICOLON . Expression_In RPAREN Statement]*
    1819u16,
    // State(1850)
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON . SEMICOLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON . SEMICOLON Expression_In RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON . Expression_In SEMICOLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON . Expression_In SEMICOLON Expression_In RPAREN Statement_Return]*
    1850u16,
    // State(1872)
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON . SEMICOLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON . SEMICOLON Expression_In_Await RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON . Expression_In_Await SEMICOLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON . Expression_In_Await SEMICOLON Expression_In_Await RPAREN Statement_Await_Return]*
    1872u16,
    // State(1924)
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMICOLON SEMICOLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMICOLON SEMICOLON . Expression_In_Await RPAREN Statement_Await]*
    1924u16,
    // State(1928)
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON Expression_In_Await SEMICOLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON Expression_In_Await SEMICOLON . Expression_In_Await RPAREN Statement_Await]*
    1928u16,
    // State(2096)
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMICOLON Expression_In SEMICOLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMICOLON Expression_In SEMICOLON . Expression_In RPAREN Statement]*
    2096u16,
    // State(2124)
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON SEMICOLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON SEMICOLON . Expression_In RPAREN Statement_Return]*
    2124u16,
    // State(2128)
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) Expression SEMICOLON . SEMICOLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) Expression SEMICOLON . SEMICOLON Expression_In RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) Expression SEMICOLON . Expression_In SEMICOLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) Expression SEMICOLON . Expression_In SEMICOLON Expression_In RPAREN Statement_Return]*
    2128u16,
    // State(2134)
    //   [ForStatement_Return -> FOR LPAREN LexicalDeclaration SEMICOLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN LexicalDeclaration SEMICOLON . Expression_In RPAREN Statement_Return]*
    2134u16,
    // State(2148)
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON SEMICOLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON SEMICOLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2148u16,
    // State(2152)
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON . SEMICOLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON . SEMICOLON Expression_In_Await RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON . Expression_In_Await SEMICOLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON . Expression_In_Await SEMICOLON Expression_In_Await RPAREN Statement_Await_Return]*
    2152u16,
    // State(2158)
    //   [ForStatement_Await_Return -> FOR LPAREN LexicalDeclaration_Await SEMICOLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN LexicalDeclaration_Await SEMICOLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2158u16,
    // State(2195)
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMICOLON Expression_In_Await SEMICOLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMICOLON Expression_In_Await SEMICOLON . Expression_In_Await RPAREN Statement_Await]*
    2195u16,
    // State(2520)
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON Expression_In SEMICOLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON Expression_In SEMICOLON . Expression_In RPAREN Statement_Return]*
    2520u16,
    // State(2523)
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMICOLON . SEMICOLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMICOLON . SEMICOLON Expression_In RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMICOLON . Expression_In SEMICOLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMICOLON . Expression_In SEMICOLON Expression_In RPAREN Statement_Return]*
    2523u16,
    // State(2524)
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) Expression SEMICOLON SEMICOLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) Expression SEMICOLON SEMICOLON . Expression_In RPAREN Statement_Return]*
    2524u16,
    // State(2532)
    //   [ForStatement_Return -> FOR LPAREN LexicalDeclaration Expression_In SEMICOLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN LexicalDeclaration Expression_In SEMICOLON . Expression_In RPAREN Statement_Return]*
    2532u16,
    // State(2545)
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON Expression_In_Await SEMICOLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON Expression_In_Await SEMICOLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2545u16,
    // State(2548)
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMICOLON . SEMICOLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMICOLON . SEMICOLON Expression_In_Await RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMICOLON . Expression_In_Await SEMICOLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMICOLON . Expression_In_Await SEMICOLON Expression_In_Await RPAREN Statement_Await_Return]*
    2548u16,
    // State(2549)
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON SEMICOLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON SEMICOLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2549u16,
    // State(2557)
    //   [ForStatement_Await_Return -> FOR LPAREN LexicalDeclaration_Await Expression_In_Await SEMICOLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN LexicalDeclaration_Await Expression_In_Await SEMICOLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2557u16,
    // State(2949)
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMICOLON SEMICOLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMICOLON SEMICOLON . Expression_In RPAREN Statement_Return]*
    2949u16,
    // State(2953)
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) Expression SEMICOLON Expression_In SEMICOLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) Expression SEMICOLON Expression_In SEMICOLON . Expression_In RPAREN Statement_Return]*
    2953u16,
    // State(2980)
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMICOLON SEMICOLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMICOLON SEMICOLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2980u16,
    // State(2984)
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON Expression_In_Await SEMICOLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMICOLON Expression_In_Await SEMICOLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2984u16,
    // State(3226)
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON . SEMICOLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON . SEMICOLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON . Expression_In_Yield SEMICOLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON . Expression_In_Yield SEMICOLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    3226u16,
    // State(3292)
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMICOLON Expression_In SEMICOLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMICOLON Expression_In SEMICOLON . Expression_In RPAREN Statement_Return]*
    3292u16,
    // State(3324)
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMICOLON Expression_In_Await SEMICOLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMICOLON Expression_In_Await SEMICOLON . Expression_In_Await RPAREN Statement_Await_Return]*
    3324u16,
    // State(3413)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON . SEMICOLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON . SEMICOLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON . Expression_In_Yield_Await SEMICOLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON . Expression_In_Yield_Await SEMICOLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3413u16,
    // State(3492)
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON SEMICOLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON SEMICOLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3492u16,
    // State(3514)
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield SEMICOLON . SEMICOLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield SEMICOLON . SEMICOLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield SEMICOLON . Expression_In_Yield SEMICOLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield SEMICOLON . Expression_In_Yield SEMICOLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    3514u16,
    // State(3530)
    //   [ForStatement_Yield_Return -> FOR LPAREN LexicalDeclaration_Yield SEMICOLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN LexicalDeclaration_Yield SEMICOLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3530u16,
    // State(3624)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON SEMICOLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON SEMICOLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3624u16,
    // State(3646)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMICOLON . SEMICOLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMICOLON . SEMICOLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMICOLON . Expression_In_Yield_Await SEMICOLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMICOLON . Expression_In_Yield_Await SEMICOLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3646u16,
    // State(3662)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN LexicalDeclaration_Yield_Await SEMICOLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN LexicalDeclaration_Yield_Await SEMICOLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3662u16,
    // State(3700)
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON Expression_In_Yield SEMICOLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON Expression_In_Yield SEMICOLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3700u16,
    // State(3707)
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMICOLON . SEMICOLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMICOLON . SEMICOLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMICOLON . Expression_In_Yield SEMICOLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMICOLON . Expression_In_Yield SEMICOLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    3707u16,
    // State(3721)
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield SEMICOLON SEMICOLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield SEMICOLON SEMICOLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3721u16,
    // State(3738)
    //   [ForStatement_Yield_Return -> FOR LPAREN LexicalDeclaration_Yield Expression_In_Yield SEMICOLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN LexicalDeclaration_Yield Expression_In_Yield SEMICOLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3738u16,
    // State(3793)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON Expression_In_Yield_Await SEMICOLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMICOLON Expression_In_Yield_Await SEMICOLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3793u16,
    // State(3800)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMICOLON . SEMICOLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMICOLON . SEMICOLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMICOLON . Expression_In_Yield_Await SEMICOLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMICOLON . Expression_In_Yield_Await SEMICOLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3800u16,
    // State(3814)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMICOLON SEMICOLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMICOLON SEMICOLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3814u16,
    // State(3831)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN LexicalDeclaration_Yield_Await Expression_In_Yield_Await SEMICOLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN LexicalDeclaration_Yield_Await Expression_In_Yield_Await SEMICOLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3831u16,
    // State(3871)
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMICOLON SEMICOLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMICOLON SEMICOLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3871u16,
    // State(3875)
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield SEMICOLON Expression_In_Yield SEMICOLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield SEMICOLON Expression_In_Yield SEMICOLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3875u16,
    // State(3923)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMICOLON SEMICOLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMICOLON SEMICOLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3923u16,
    // State(3927)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMICOLON Expression_In_Yield_Await SEMICOLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMICOLON Expression_In_Yield_Await SEMICOLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3927u16,
    // State(3966)
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMICOLON Expression_In_Yield SEMICOLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMICOLON Expression_In_Yield SEMICOLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3966u16,
    // State(4009)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMICOLON Expression_In_Yield_Await SEMICOLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMICOLON Expression_In_Yield_Await SEMICOLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    4009u16,
};

pub const DO_WHITES: Set<u16> = phf_set! {
    // State(1799)
    //   [DoWhileStatement -> DO Statement WHILE LPAREN Expression_In RPAREN . SEMICOLON]*
    1799u16,
    // State(1901)
    //   [DoWhileStatement_Await -> DO Statement_Await WHILE LPAREN Expression_In_Await RPAREN . SEMICOLON]*
    1901u16,
    // State(2942)
    //   [DoWhileStatement_Return -> DO Statement_Return WHILE LPAREN Expression_In RPAREN . SEMICOLON]*
    2942u16,
    // State(2970)
    //   [DoWhileStatement_Await_Return -> DO Statement_Await_Return WHILE LPAREN Expression_In_Await RPAREN . SEMICOLON]*
    2970u16,
    // State(3859)
    //   [DoWhileStatement_Yield_Return -> DO Statement_Yield_Return WHILE LPAREN Expression_In_Yield RPAREN . SEMICOLON]*
    3859u16,
    // State(3908)
    //   [DoWhileStatement_Yield_Await_Return -> DO Statement_Yield_Await_Return WHILE LPAREN Expression_In_Yield_Await RPAREN . SEMICOLON]*
    3908u16,
};
