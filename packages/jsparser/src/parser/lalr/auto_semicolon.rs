// DO NOT EDIT THIS FILE BY HAND.
//
// This file was automagically generated by using lalr/auto_semicolon.rs.hbs.

use phf::phf_set;
use phf::Set;

pub const DISALLOWED: Set<u16> = phf_set! {
    // State(42)
    //   [EmptyStatement -> SEMI_COLON .]*
    42u16,
    // State(417)
    //   [ForStatement -> FOR LPAREN SEMI_COLON . SEMI_COLON RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN SEMI_COLON . SEMI_COLON Expression_In RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN SEMI_COLON . Expression_In SEMI_COLON RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN SEMI_COLON . Expression_In SEMI_COLON Expression_In RPAREN Statement]*
    417u16,
    // State(701)
    //   [ForStatement -> FOR LPAREN SEMI_COLON SEMI_COLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN SEMI_COLON SEMI_COLON . Expression_In RPAREN Statement]*
    701u16,
    // State(719)
    //   [ForStatement -> FOR LPAREN Expression SEMI_COLON . SEMI_COLON RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN Expression SEMI_COLON . SEMI_COLON Expression_In RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN Expression SEMI_COLON . Expression_In SEMI_COLON RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN Expression SEMI_COLON . Expression_In SEMI_COLON Expression_In RPAREN Statement]*
    719u16,
    // State(735)
    //   [ForStatement -> FOR LPAREN LexicalDeclaration SEMI_COLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN LexicalDeclaration SEMI_COLON . Expression_In RPAREN Statement]*
    735u16,
    // State(1048)
    //   [ForStatement -> FOR LPAREN SEMI_COLON Expression_In SEMI_COLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN SEMI_COLON Expression_In SEMI_COLON . Expression_In RPAREN Statement]*
    1048u16,
    // State(1055)
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON . SEMI_COLON RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON . SEMI_COLON Expression_In RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON . Expression_In SEMI_COLON RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON . Expression_In SEMI_COLON Expression_In RPAREN Statement]*
    1055u16,
    // State(1092)
    //   [ForStatement -> FOR LPAREN Expression SEMI_COLON SEMI_COLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN Expression SEMI_COLON SEMI_COLON . Expression_In RPAREN Statement]*
    1092u16,
    // State(1109)
    //   [ForStatement -> FOR LPAREN LexicalDeclaration Expression_In SEMI_COLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN LexicalDeclaration Expression_In SEMI_COLON . Expression_In RPAREN Statement]*
    1109u16,
    // State(1445)
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON SEMI_COLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON SEMI_COLON . Expression_In RPAREN Statement]*
    1445u16,
    // State(1477)
    //   [ForStatement -> FOR LPAREN Expression SEMI_COLON Expression_In SEMI_COLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN Expression SEMI_COLON Expression_In SEMI_COLON . Expression_In RPAREN Statement]*
    1477u16,
    // State(1508)
    //   [ForStatement_Return -> FOR LPAREN SEMI_COLON . SEMI_COLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN SEMI_COLON . SEMI_COLON Expression_In RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN SEMI_COLON . Expression_In SEMI_COLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN SEMI_COLON . Expression_In SEMI_COLON Expression_In RPAREN Statement_Return]*
    1508u16,
    // State(1557)
    //   [ForStatement_Await_Return -> FOR LPAREN SEMI_COLON . SEMI_COLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN SEMI_COLON . SEMI_COLON Expression_In_Await RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN SEMI_COLON . Expression_In_Await SEMI_COLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN SEMI_COLON . Expression_In_Await SEMI_COLON Expression_In_Await RPAREN Statement_Await_Return]*
    1557u16,
    // State(1779)
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON Expression_In SEMI_COLON . RPAREN Statement]*
    //   [ForStatement -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON Expression_In SEMI_COLON . Expression_In RPAREN Statement]*
    1779u16,
    // State(1832)
    //   [ForStatement_Return -> FOR LPAREN SEMI_COLON SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN SEMI_COLON SEMI_COLON . Expression_In RPAREN Statement_Return]*
    1832u16,
    // State(1836)
    //   [ForStatement_Return -> FOR LPAREN Expression SEMI_COLON . SEMI_COLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN Expression SEMI_COLON . SEMI_COLON Expression_In RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN Expression SEMI_COLON . Expression_In SEMI_COLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN Expression SEMI_COLON . Expression_In SEMI_COLON Expression_In RPAREN Statement_Return]*
    1836u16,
    // State(1842)
    //   [ForStatement_Return -> FOR LPAREN LexicalDeclaration SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN LexicalDeclaration SEMI_COLON . Expression_In RPAREN Statement_Return]*
    1842u16,
    // State(1872)
    //   [ForStatement_Await_Return -> FOR LPAREN SEMI_COLON SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN SEMI_COLON SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    1872u16,
    // State(1880)
    //   [ForStatement_Await_Return -> FOR LPAREN Expression_Await SEMI_COLON . SEMI_COLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN Expression_Await SEMI_COLON . SEMI_COLON Expression_In_Await RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN Expression_Await SEMI_COLON . Expression_In_Await SEMI_COLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN Expression_Await SEMI_COLON . Expression_In_Await SEMI_COLON Expression_In_Await RPAREN Statement_Await_Return]*
    1880u16,
    // State(1891)
    //   [ForStatement_Await_Return -> FOR LPAREN LexicalDeclaration_Await SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN LexicalDeclaration_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    1891u16,
    // State(2021)
    //   [ForStatement_Await -> FOR LPAREN SEMI_COLON . SEMI_COLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN SEMI_COLON . SEMI_COLON Expression_In_Await RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN SEMI_COLON . Expression_In_Await SEMI_COLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN SEMI_COLON . Expression_In_Await SEMI_COLON Expression_In_Await RPAREN Statement_Await]*
    2021u16,
    // State(2251)
    //   [ForStatement_Return -> FOR LPAREN SEMI_COLON Expression_In SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN SEMI_COLON Expression_In SEMI_COLON . Expression_In RPAREN Statement_Return]*
    2251u16,
    // State(2254)
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON . SEMI_COLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON . SEMI_COLON Expression_In RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON . Expression_In SEMI_COLON RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON . Expression_In SEMI_COLON Expression_In RPAREN Statement_Return]*
    2254u16,
    // State(2255)
    //   [ForStatement_Return -> FOR LPAREN Expression SEMI_COLON SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN Expression SEMI_COLON SEMI_COLON . Expression_In RPAREN Statement_Return]*
    2255u16,
    // State(2263)
    //   [ForStatement_Return -> FOR LPAREN LexicalDeclaration Expression_In SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN LexicalDeclaration Expression_In SEMI_COLON . Expression_In RPAREN Statement_Return]*
    2263u16,
    // State(2286)
    //   [ForStatement_Await_Return -> FOR LPAREN SEMI_COLON Expression_In_Await SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN SEMI_COLON Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2286u16,
    // State(2293)
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . SEMI_COLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . SEMI_COLON Expression_In_Await RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . Expression_In_Await SEMI_COLON RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . Expression_In_Await SEMI_COLON Expression_In_Await RPAREN Statement_Await_Return]*
    2293u16,
    // State(2295)
    //   [ForStatement_Await_Return -> FOR LPAREN Expression_Await SEMI_COLON SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN Expression_Await SEMI_COLON SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2295u16,
    // State(2307)
    //   [ForStatement_Await_Return -> FOR LPAREN LexicalDeclaration_Await Expression_In_Await SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN LexicalDeclaration_Await Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2307u16,
    // State(2509)
    //   [ForStatement_Await -> FOR LPAREN SEMI_COLON SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN SEMI_COLON SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    2509u16,
    // State(2513)
    //   [ForStatement_Await -> FOR LPAREN Expression_Await SEMI_COLON . SEMI_COLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN Expression_Await SEMI_COLON . SEMI_COLON Expression_In_Await RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN Expression_Await SEMI_COLON . Expression_In_Await SEMI_COLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN Expression_Await SEMI_COLON . Expression_In_Await SEMI_COLON Expression_In_Await RPAREN Statement_Await]*
    2513u16,
    // State(2519)
    //   [ForStatement_Await -> FOR LPAREN LexicalDeclaration_Await SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN LexicalDeclaration_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    2519u16,
    // State(2698)
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON SEMI_COLON . Expression_In RPAREN Statement_Return]*
    2698u16,
    // State(2702)
    //   [ForStatement_Return -> FOR LPAREN Expression SEMI_COLON Expression_In SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN Expression SEMI_COLON Expression_In SEMI_COLON . Expression_In RPAREN Statement_Return]*
    2702u16,
    // State(2741)
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2741u16,
    // State(2745)
    //   [ForStatement_Await_Return -> FOR LPAREN Expression_Await SEMI_COLON Expression_In_Await SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN Expression_Await SEMI_COLON Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    2745u16,
    // State(2920)
    //   [ForStatement_Await -> FOR LPAREN SEMI_COLON Expression_In_Await SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN SEMI_COLON Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    2920u16,
    // State(2923)
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . SEMI_COLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . SEMI_COLON Expression_In_Await RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . Expression_In_Await SEMI_COLON RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON . Expression_In_Await SEMI_COLON Expression_In_Await RPAREN Statement_Await]*
    2923u16,
    // State(2924)
    //   [ForStatement_Await -> FOR LPAREN Expression_Await SEMI_COLON SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN Expression_Await SEMI_COLON SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    2924u16,
    // State(2932)
    //   [ForStatement_Await -> FOR LPAREN LexicalDeclaration_Await Expression_In_Await SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN LexicalDeclaration_Await Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    2932u16,
    // State(2993)
    //   [ForStatement_Yield_Return -> FOR LPAREN SEMI_COLON . SEMI_COLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN SEMI_COLON . SEMI_COLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN SEMI_COLON . Expression_In_Yield SEMI_COLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN SEMI_COLON . Expression_In_Yield SEMI_COLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    2993u16,
    // State(3059)
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON Expression_In SEMI_COLON . RPAREN Statement_Return]*
    //   [ForStatement_Return -> FOR LPAREN VAR VariableDeclarationList SEMI_COLON Expression_In SEMI_COLON . Expression_In RPAREN Statement_Return]*
    3059u16,
    // State(3099)
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON Expression_In_Await SEMI_COLON . RPAREN Statement_Await_Return]*
    //   [ForStatement_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await_Return]*
    3099u16,
    // State(3177)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN SEMI_COLON . SEMI_COLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN SEMI_COLON . SEMI_COLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN SEMI_COLON . Expression_In_Yield_Await SEMI_COLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN SEMI_COLON . Expression_In_Yield_Await SEMI_COLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3177u16,
    // State(3239)
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    3239u16,
    // State(3243)
    //   [ForStatement_Await -> FOR LPAREN Expression_Await SEMI_COLON Expression_In_Await SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN Expression_Await SEMI_COLON Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    3243u16,
    // State(3287)
    //   [ForStatement_Yield_Return -> FOR LPAREN SEMI_COLON SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN SEMI_COLON SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3287u16,
    // State(3309)
    //   [ForStatement_Yield_Return -> FOR LPAREN Expression_Yield SEMI_COLON . SEMI_COLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN Expression_Yield SEMI_COLON . SEMI_COLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN Expression_Yield SEMI_COLON . Expression_In_Yield SEMI_COLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN Expression_Yield SEMI_COLON . Expression_In_Yield SEMI_COLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    3309u16,
    // State(3325)
    //   [ForStatement_Yield_Return -> FOR LPAREN LexicalDeclaration_Yield SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN LexicalDeclaration_Yield SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3325u16,
    // State(3424)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN SEMI_COLON SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN SEMI_COLON SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3424u16,
    // State(3446)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN Expression_Yield_Await SEMI_COLON . SEMI_COLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN Expression_Yield_Await SEMI_COLON . SEMI_COLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN Expression_Yield_Await SEMI_COLON . Expression_In_Yield_Await SEMI_COLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN Expression_Yield_Await SEMI_COLON . Expression_In_Yield_Await SEMI_COLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3446u16,
    // State(3462)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN LexicalDeclaration_Yield_Await SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN LexicalDeclaration_Yield_Await SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3462u16,
    // State(3495)
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON Expression_In_Await SEMI_COLON . RPAREN Statement_Await]*
    //   [ForStatement_Await -> FOR LPAREN VAR VariableDeclarationList_Await SEMI_COLON Expression_In_Await SEMI_COLON . Expression_In_Await RPAREN Statement_Await]*
    3495u16,
    // State(3532)
    //   [ForStatement_Yield_Return -> FOR LPAREN SEMI_COLON Expression_In_Yield SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN SEMI_COLON Expression_In_Yield SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3532u16,
    // State(3539)
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON . SEMI_COLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON . SEMI_COLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON . Expression_In_Yield SEMI_COLON RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON . Expression_In_Yield SEMI_COLON Expression_In_Yield RPAREN Statement_Yield_Return]*
    3539u16,
    // State(3553)
    //   [ForStatement_Yield_Return -> FOR LPAREN Expression_Yield SEMI_COLON SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN Expression_Yield SEMI_COLON SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3553u16,
    // State(3570)
    //   [ForStatement_Yield_Return -> FOR LPAREN LexicalDeclaration_Yield Expression_In_Yield SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN LexicalDeclaration_Yield Expression_In_Yield SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3570u16,
    // State(3631)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN SEMI_COLON Expression_In_Yield_Await SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN SEMI_COLON Expression_In_Yield_Await SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3631u16,
    // State(3638)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON . SEMI_COLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON . SEMI_COLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON . Expression_In_Yield_Await SEMI_COLON RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON . Expression_In_Yield_Await SEMI_COLON Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3638u16,
    // State(3652)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN Expression_Yield_Await SEMI_COLON SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN Expression_Yield_Await SEMI_COLON SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3652u16,
    // State(3669)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN LexicalDeclaration_Yield_Await Expression_In_Yield_Await SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN LexicalDeclaration_Yield_Await Expression_In_Yield_Await SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3669u16,
    // State(3728)
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3728u16,
    // State(3732)
    //   [ForStatement_Yield_Return -> FOR LPAREN Expression_Yield SEMI_COLON Expression_In_Yield SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN Expression_Yield SEMI_COLON Expression_In_Yield SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3732u16,
    // State(3782)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3782u16,
    // State(3786)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN Expression_Yield_Await SEMI_COLON Expression_In_Yield_Await SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN Expression_Yield_Await SEMI_COLON Expression_In_Yield_Await SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3786u16,
    // State(3832)
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON Expression_In_Yield SEMI_COLON . RPAREN Statement_Yield_Return]*
    //   [ForStatement_Yield_Return -> FOR LPAREN VAR VariableDeclarationList_Yield SEMI_COLON Expression_In_Yield SEMI_COLON . Expression_In_Yield RPAREN Statement_Yield_Return]*
    3832u16,
    // State(3875)
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON Expression_In_Yield_Await SEMI_COLON . RPAREN Statement_Yield_Await_Return]*
    //   [ForStatement_Yield_Await_Return -> FOR LPAREN VAR VariableDeclarationList_Yield_Await SEMI_COLON Expression_In_Yield_Await SEMI_COLON . Expression_In_Yield_Await RPAREN Statement_Yield_Await_Return]*
    3875u16,
};

pub const DO_WHITES: Set<u16> = phf_set! {
    // State(1433)
    //   [DoWhileStatement -> DO Statement WHILE LPAREN Expression_In RPAREN . SEMI_COLON]*
    1433u16,
    // State(2691)
    //   [DoWhileStatement_Return -> DO Statement_Return WHILE LPAREN Expression_In RPAREN . SEMI_COLON]*
    2691u16,
    // State(2727)
    //   [DoWhileStatement_Await_Return -> DO Statement_Await_Return WHILE LPAREN Expression_In_Await RPAREN . SEMI_COLON]*
    2727u16,
    // State(3229)
    //   [DoWhileStatement_Await -> DO Statement_Await WHILE LPAREN Expression_In_Await RPAREN . SEMI_COLON]*
    3229u16,
    // State(3716)
    //   [DoWhileStatement_Yield_Return -> DO Statement_Yield_Return WHILE LPAREN Expression_In_Yield RPAREN . SEMI_COLON]*
    3716u16,
    // State(3767)
    //   [DoWhileStatement_Yield_Await_Return -> DO Statement_Yield_Await_Return WHILE LPAREN Expression_In_Yield_Await RPAREN . SEMI_COLON]*
    3767u16,
};
