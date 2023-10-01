// DO NOT EDIT THIS FILE BY HAND.
//
// This file was automagically generated by:
// codegen.js --input-stdin --no-escape lalr/auto_semicolon.rs.hbs

use phf::phf_set;
use phf::Set;

pub const DISALLOWED: Set<u16> = phf_set! {
    // State(43)
    //   [EmptyStatement -> SEMI_COLON .]*
    43u16,
    // State(468)
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . SEMI_COLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . SEMI_COLON Expression_In_Await RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . Expression_In_Await SEMI_COLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . Expression_In_Await SEMI_COLON Expression_In_Await RPAREN Statement_Await]*
    468u16,
    // State(771)
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    771u16,
    // State(789)
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON . SEMI_COLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON . SEMI_COLON Expression_In_Await RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON . Expression_In_Await SEMI_COLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON . Expression_In_Await SEMI_COLON Expression_In_Await RPAREN Statement_Await]*
    789u16,
    // State(805)
    //   [ForStatement_Await -> FOR LPAREN LexicalDeclaration_Await SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN LexicalDeclaration_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    805u16,
    // State(1159)
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON Expression_In_Await SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    1159u16,
    // State(1166)
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . SEMI_COLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . SEMI_COLON Expression_In_Await RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . Expression_In_Await SEMI_COLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . Expression_In_Await SEMI_COLON Expression_In_Await RPAREN Statement_Await]*
    1166u16,
    // State(1203)
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    1203u16,
    // State(1220)
    //   [ForStatement_Await -> FOR LPAREN LexicalDeclaration_Await Expression_In_Await SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN LexicalDeclaration_Await Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    1220u16,
    // State(1532)
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    1532u16,
    // State(1564)
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON Expression_In_Await SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    1564u16,
    // State(1621)
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . SEMI_COLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . SEMI_COLON Expression_In RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . Expression_In SEMI_COLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . Expression_In SEMI_COLON Expression_In RPAREN Statement_Return]*
    1621u16,
    // State(1684)
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . SEMI_COLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . SEMI_COLON Expression_In_Await RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . Expression_In_Await SEMI_COLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . Expression_In_Await SEMI_COLON Expression_In_Await RPAREN Statement_Await_Return]*
    1684u16,
    // State(1861)
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON Expression_In_Await SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    1861u16,
    // State(1927)
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON SEMI_COLON . Expression_In RPAREN Statement_Return]*
    1927u16,
    // State(1935)
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) Expression SEMI_COLON . SEMI_COLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) Expression SEMI_COLON . SEMI_COLON Expression_In RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) Expression SEMI_COLON . Expression_In SEMI_COLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) Expression SEMI_COLON . Expression_In SEMI_COLON Expression_In RPAREN Statement_Return]*
    1935u16,
    // State(1946)
    //   [ForStatement_Return -> FOR LPAREN LexicalDeclaration SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN LexicalDeclaration SEMI_COLON . Expression_In RPAREN Statement_Return]*
    1946u16,
    // State(1979)
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    1979u16,
    // State(1983)
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON . SEMI_COLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON . SEMI_COLON Expression_In_Await RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON . Expression_In_Await SEMI_COLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON . Expression_In_Await SEMI_COLON Expression_In_Await RPAREN Statement_Await_Return]*
    1983u16,
    // State(1989)
    //   [ForStatement_Await_Return -> FOR LPAREN LexicalDeclaration_Await SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN LexicalDeclaration_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    1989u16,
    // State(2324)
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON Expression_In SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON Expression_In SEMI_COLON . Expression_In RPAREN Statement_Return]*
    2324u16,
    // State(2331)
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON . SEMI_COLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON . SEMI_COLON Expression_In RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON . Expression_In SEMI_COLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON . Expression_In SEMI_COLON Expression_In RPAREN Statement_Return]*
    2331u16,
    // State(2333)
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) Expression SEMI_COLON SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) Expression SEMI_COLON SEMI_COLON . Expression_In RPAREN Statement_Return]*
    2333u16,
    // State(2345)
    //   [ForStatement_Return -> FOR LPAREN LexicalDeclaration Expression_In SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN LexicalDeclaration Expression_In SEMI_COLON . Expression_In RPAREN Statement_Return]*
    2345u16,
    // State(2366)
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON Expression_In_Await SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2366u16,
    // State(2369)
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . SEMI_COLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . SEMI_COLON Expression_In_Await RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . Expression_In_Await SEMI_COLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . Expression_In_Await SEMI_COLON Expression_In_Await RPAREN Statement_Await_Return]*
    2369u16,
    // State(2370)
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2370u16,
    // State(2378)
    //   [ForStatement_Await_Return -> FOR LPAREN LexicalDeclaration_Await Expression_In_Await SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN LexicalDeclaration_Await Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2378u16,
    // State(2756)
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON SEMI_COLON . Expression_In RPAREN Statement_Return]*
    2756u16,
    // State(2760)
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) Expression SEMI_COLON Expression_In SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN (?![LET LBRACK]) Expression SEMI_COLON Expression_In SEMI_COLON . Expression_In RPAREN Statement_Return]*
    2760u16,
    // State(2792)
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2792u16,
    // State(2796)
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON Expression_In_Await SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Await SEMI_COLON Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2796u16,
    // State(3014)
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . SEMI_COLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . SEMI_COLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . Expression_In_Yield SEMI_COLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . Expression_In_Yield SEMI_COLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    3014u16,
    // State(3095)
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON Expression_In SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON Expression_In SEMI_COLON . Expression_In RPAREN Statement_Return]*
    3095u16,
    // State(3129)
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON Expression_In_Await SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    3129u16,
    // State(3205)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . SEMI_COLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . SEMI_COLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . Expression_In_Yield_Await SEMI_COLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON . Expression_In_Yield_Await SEMI_COLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3205u16,
    // State(3284)
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3284u16,
    // State(3306)
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield SEMI_COLON . SEMI_COLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield SEMI_COLON . SEMI_COLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield SEMI_COLON . Expression_In_Yield SEMI_COLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield SEMI_COLON . Expression_In_Yield SEMI_COLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    3306u16,
    // State(3322)
    //   [ForStatement_Yield_Return -> FOR LPAREN LexicalDeclaration_Yield SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN LexicalDeclaration_Yield SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3322u16,
    // State(3425)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3425u16,
    // State(3447)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMI_COLON . SEMI_COLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMI_COLON . SEMI_COLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMI_COLON . Expression_In_Yield_Await SEMI_COLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMI_COLON . Expression_In_Yield_Await SEMI_COLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3447u16,
    // State(3463)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN LexicalDeclaration_Yield_Await SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN LexicalDeclaration_Yield_Await SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3463u16,
    // State(3501)
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON Expression_In_Yield SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON Expression_In_Yield SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3501u16,
    // State(3508)
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON . SEMI_COLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON . SEMI_COLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON . Expression_In_Yield SEMI_COLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON . Expression_In_Yield SEMI_COLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    3508u16,
    // State(3522)
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield SEMI_COLON SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield SEMI_COLON SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3522u16,
    // State(3539)
    //   [ForStatement_Yield_Return -> FOR LPAREN LexicalDeclaration_Yield Expression_In_Yield SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN LexicalDeclaration_Yield Expression_In_Yield SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3539u16,
    // State(3601)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON Expression_In_Yield_Await SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) SEMI_COLON Expression_In_Yield_Await SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3601u16,
    // State(3608)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON . SEMI_COLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON . SEMI_COLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON . Expression_In_Yield_Await SEMI_COLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON . Expression_In_Yield_Await SEMI_COLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3608u16,
    // State(3622)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMI_COLON SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMI_COLON SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3622u16,
    // State(3639)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN LexicalDeclaration_Yield_Await Expression_In_Yield_Await SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN LexicalDeclaration_Yield_Await Expression_In_Yield_Await SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3639u16,
    // State(3679)
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3679u16,
    // State(3683)
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield SEMI_COLON Expression_In_Yield SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield SEMI_COLON Expression_In_Yield SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3683u16,
    // State(3733)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3733u16,
    // State(3737)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMI_COLON Expression_In_Yield_Await SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN (?![LET LBRACK]) Expression_Yield_Await SEMI_COLON Expression_In_Yield_Await SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3737u16,
    // State(3776)
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON Expression_In_Yield SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON Expression_In_Yield SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3776u16,
    // State(3819)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON Expression_In_Yield_Await SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON Expression_In_Yield_Await SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3819u16,
};

pub const DO_WHITES: Set<u16> = phf_set! {
    // State(1509)
    //   [DoWhileStatement_Await -> DO Statement_Await WHILE LPAREN Expression_In_Await RPAREN . SEMI_COLON]*
    1509u16,
    // State(2745)
    //   [DoWhileStatement_Return -> DO Statement_Return WHILE LPAREN Expression_In RPAREN . SEMI_COLON]*
    2745u16,
    // State(2782)
    //   [DoWhileStatement_Await_Return -> DO Statement_Await_Return WHILE LPAREN Expression_In_Await RPAREN . SEMI_COLON]*
    2782u16,
    // State(3667)
    //   [DoWhileStatement_Yield_Return -> DO Statement_Yield_Return WHILE LPAREN Expression_In_Yield RPAREN . SEMI_COLON]*
    3667u16,
    // State(3718)
    //   [DoWhileStatement_Yield_Await_Return -> DO Statement_Yield_Await_Return WHILE LPAREN Expression_In_Yield_Await RPAREN . SEMI_COLON]*
    3718u16,
};
