// DO NOT EDIT THIS FILE BY HAND.
//
// This file was automagically generated with:
// template: libs/jsparser/src/parser/lalr/non_terminals.rs.hbs

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum NonTerminal {
    AdditiveExpression,
    AdditiveExpression_Await,
    AdditiveExpression_Yield,
    AdditiveExpression_Yield_Await,
    ArgumentList,
    ArgumentList_Await,
    ArgumentList_Yield,
    ArgumentList_Yield_Await,
    Arguments,
    Arguments_Await,
    Arguments_Yield,
    Arguments_Yield_Await,
    ArrayBindingPattern,
    ArrayBindingPattern_Await,
    ArrayBindingPattern_Yield,
    ArrayBindingPattern_Yield_Await,
    ArrayLiteral,
    ArrayLiteral_Await,
    ArrayLiteral_Yield,
    ArrayLiteral_Yield_Await,
    ArrowFunction,
    ArrowFunction_Await,
    ArrowFunction_In,
    ArrowFunction_In_Await,
    ArrowFunction_In_Yield,
    ArrowFunction_In_Yield_Await,
    ArrowFunction_Yield,
    ArrowFunction_Yield_Await,
    ArrowParameters,
    ArrowParameters_Await,
    ArrowParameters_Yield,
    ArrowParameters_Yield_Await,
    AssignmentExpression,
    AssignmentExpression_Await,
    AssignmentExpression_In,
    AssignmentExpression_In_Await,
    AssignmentExpression_In_Yield,
    AssignmentExpression_In_Yield_Await,
    AssignmentExpression_Yield,
    AssignmentExpression_Yield_Await,
    AssignmentOperator,
    AsyncArrowBindingIdentifier,
    AsyncArrowBindingIdentifier_Yield,
    AsyncArrowFunction,
    AsyncArrowFunction_Await,
    AsyncArrowFunction_In,
    AsyncArrowFunction_In_Await,
    AsyncArrowFunction_In_Yield,
    AsyncArrowFunction_In_Yield_Await,
    AsyncArrowFunction_Yield,
    AsyncArrowFunction_Yield_Await,
    AsyncConciseBody,
    AsyncConciseBody_In,
    AsyncFunctionBody,
    AsyncFunctionDeclaration,
    AsyncFunctionDeclaration_Await,
    AsyncFunctionDeclaration_Await_Default,
    AsyncFunctionDeclaration_Yield,
    AsyncFunctionDeclaration_Yield_Await,
    AsyncFunctionExpression,
    AsyncGeneratorBody,
    AsyncGeneratorDeclaration,
    AsyncGeneratorDeclaration_Await,
    AsyncGeneratorDeclaration_Await_Default,
    AsyncGeneratorDeclaration_Yield,
    AsyncGeneratorDeclaration_Yield_Await,
    AsyncGeneratorExpression,
    AsyncGeneratorMethod,
    AsyncGeneratorMethod_Await,
    AsyncGeneratorMethod_Yield,
    AsyncGeneratorMethod_Yield_Await,
    AsyncMethod,
    AsyncMethod_Await,
    AsyncMethod_Yield,
    AsyncMethod_Yield_Await,
    AwaitExpression,
    AwaitExpression_Yield,
    BindingElement,
    BindingElementList,
    BindingElementList_Await,
    BindingElementList_Yield,
    BindingElementList_Yield_Await,
    BindingElement_Await,
    BindingElement_Yield,
    BindingElement_Yield_Await,
    BindingElisionElement,
    BindingElisionElement_Await,
    BindingElisionElement_Yield,
    BindingElisionElement_Yield_Await,
    BindingIdentifier,
    BindingIdentifier_Await,
    BindingIdentifier_Yield,
    BindingIdentifier_Yield_Await,
    BindingList,
    BindingList_Await,
    BindingList_In,
    BindingList_In_Await,
    BindingList_In_Yield,
    BindingList_In_Yield_Await,
    BindingList_Yield,
    BindingList_Yield_Await,
    BindingPattern,
    BindingPattern_Await,
    BindingPattern_Yield,
    BindingPattern_Yield_Await,
    BindingProperty,
    BindingPropertyList,
    BindingPropertyList_Await,
    BindingPropertyList_Yield,
    BindingPropertyList_Yield_Await,
    BindingProperty_Await,
    BindingProperty_Yield,
    BindingProperty_Yield_Await,
    BindingRestElement,
    BindingRestElement_Await,
    BindingRestElement_Yield,
    BindingRestElement_Yield_Await,
    BindingRestProperty,
    BindingRestProperty_Await,
    BindingRestProperty_Yield,
    BindingRestProperty_Yield_Await,
    BitwiseANDExpression,
    BitwiseANDExpression_Await,
    BitwiseANDExpression_In,
    BitwiseANDExpression_In_Await,
    BitwiseANDExpression_In_Yield,
    BitwiseANDExpression_In_Yield_Await,
    BitwiseANDExpression_Yield,
    BitwiseANDExpression_Yield_Await,
    BitwiseORExpression,
    BitwiseORExpression_Await,
    BitwiseORExpression_In,
    BitwiseORExpression_In_Await,
    BitwiseORExpression_In_Yield,
    BitwiseORExpression_In_Yield_Await,
    BitwiseORExpression_Yield,
    BitwiseORExpression_Yield_Await,
    BitwiseXORExpression,
    BitwiseXORExpression_Await,
    BitwiseXORExpression_In,
    BitwiseXORExpression_In_Await,
    BitwiseXORExpression_In_Yield,
    BitwiseXORExpression_In_Yield_Await,
    BitwiseXORExpression_Yield,
    BitwiseXORExpression_Yield_Await,
    Block,
    BlockStatement,
    BlockStatement_Await,
    BlockStatement_Await_Return,
    BlockStatement_Return,
    BlockStatement_Yield_Await_Return,
    BlockStatement_Yield_Return,
    Block_Await,
    Block_Await_Return,
    Block_Return,
    Block_Yield_Await_Return,
    Block_Yield_Return,
    BooleanLiteral,
    BreakStatement,
    BreakStatement_Await,
    BreakStatement_Yield,
    BreakStatement_Yield_Await,
    BreakableStatement,
    BreakableStatement_Await,
    BreakableStatement_Await_Return,
    BreakableStatement_Return,
    BreakableStatement_Yield_Await_Return,
    BreakableStatement_Yield_Return,
    CallExpression,
    CallExpression_Await,
    CallExpression_Yield,
    CallExpression_Yield_Await,
    CaseBlock,
    CaseBlock_Await,
    CaseBlock_Await_Return,
    CaseBlock_Return,
    CaseBlock_Yield_Await_Return,
    CaseBlock_Yield_Return,
    CaseClause,
    CaseClause_Await,
    CaseClause_Await_Return,
    CaseClause_Return,
    CaseClause_Yield_Await_Return,
    CaseClause_Yield_Return,
    CaseClauses,
    CaseClauses_Await,
    CaseClauses_Await_Return,
    CaseClauses_Return,
    CaseClauses_Yield_Await_Return,
    CaseClauses_Yield_Return,
    Catch,
    CatchParameter,
    CatchParameter_Await,
    CatchParameter_Yield,
    CatchParameter_Yield_Await,
    Catch_Await,
    Catch_Await_Return,
    Catch_Return,
    Catch_Yield_Await_Return,
    Catch_Yield_Return,
    ClassBody,
    ClassBody_Await,
    ClassBody_Yield,
    ClassBody_Yield_Await,
    ClassDeclaration,
    ClassDeclaration_Await,
    ClassDeclaration_Await_Default,
    ClassDeclaration_Yield,
    ClassDeclaration_Yield_Await,
    ClassElement,
    ClassElementList,
    ClassElementList_Await,
    ClassElementList_Yield,
    ClassElementList_Yield_Await,
    ClassElementName,
    ClassElementName_Await,
    ClassElementName_Yield,
    ClassElementName_Yield_Await,
    ClassElement_Await,
    ClassElement_Yield,
    ClassElement_Yield_Await,
    ClassExpression,
    ClassExpression_Await,
    ClassExpression_Yield,
    ClassExpression_Yield_Await,
    ClassHeritage,
    ClassHeritage_Await,
    ClassHeritage_Yield,
    ClassHeritage_Yield_Await,
    ClassStaticBlock,
    ClassStaticBlockBody,
    ClassStaticBlockStatementList,
    ClassTail,
    ClassTail_Await,
    ClassTail_Yield,
    ClassTail_Yield_Await,
    CoalesceExpression,
    CoalesceExpressionHead,
    CoalesceExpressionHead_Await,
    CoalesceExpressionHead_In,
    CoalesceExpressionHead_In_Await,
    CoalesceExpressionHead_In_Yield,
    CoalesceExpressionHead_In_Yield_Await,
    CoalesceExpressionHead_Yield,
    CoalesceExpressionHead_Yield_Await,
    CoalesceExpression_Await,
    CoalesceExpression_In,
    CoalesceExpression_In_Await,
    CoalesceExpression_In_Yield,
    CoalesceExpression_In_Yield_Await,
    CoalesceExpression_Yield,
    CoalesceExpression_Yield_Await,
    ComputedPropertyName,
    ComputedPropertyName_Await,
    ComputedPropertyName_Yield,
    ComputedPropertyName_Yield_Await,
    ConciseBody,
    ConciseBody_In,
    ConditionalExpression,
    ConditionalExpression_Await,
    ConditionalExpression_In,
    ConditionalExpression_In_Await,
    ConditionalExpression_In_Yield,
    ConditionalExpression_In_Yield_Await,
    ConditionalExpression_Yield,
    ConditionalExpression_Yield_Await,
    ContinueStatement,
    ContinueStatement_Await,
    ContinueStatement_Yield,
    ContinueStatement_Yield_Await,
    CoverCallExpressionAndAsyncArrowHead,
    CoverCallExpressionAndAsyncArrowHead_Await,
    CoverCallExpressionAndAsyncArrowHead_Yield,
    CoverCallExpressionAndAsyncArrowHead_Yield_Await,
    CoverInitializedName,
    CoverInitializedName_Await,
    CoverInitializedName_Yield,
    CoverInitializedName_Yield_Await,
    CoverParenthesizedExpressionAndArrowParameterList,
    CoverParenthesizedExpressionAndArrowParameterList_Await,
    CoverParenthesizedExpressionAndArrowParameterList_Yield,
    CoverParenthesizedExpressionAndArrowParameterList_Yield_Await,
    DebuggerStatement,
    Declaration,
    Declaration_Await,
    Declaration_Yield,
    Declaration_Yield_Await,
    DefaultClause,
    DefaultClause_Await,
    DefaultClause_Await_Return,
    DefaultClause_Return,
    DefaultClause_Yield_Await_Return,
    DefaultClause_Yield_Return,
    DoWhileStatement,
    DoWhileStatement_Await,
    DoWhileStatement_Await_Return,
    DoWhileStatement_Return,
    DoWhileStatement_Yield_Await_Return,
    DoWhileStatement_Yield_Return,
    ElementList,
    ElementList_Await,
    ElementList_Yield,
    ElementList_Yield_Await,
    Elision,
    EmptyStatement,
    EqualityExpression,
    EqualityExpression_Await,
    EqualityExpression_In,
    EqualityExpression_In_Await,
    EqualityExpression_In_Yield,
    EqualityExpression_In_Yield_Await,
    EqualityExpression_Yield,
    EqualityExpression_Yield_Await,
    ExponentiationExpression,
    ExponentiationExpression_Await,
    ExponentiationExpression_Yield,
    ExponentiationExpression_Yield_Await,
    ExportDeclaration,
    ExportFromClause,
    ExportSpecifier,
    ExportsList,
    Expression,
    ExpressionBody,
    ExpressionBody_Await,
    ExpressionBody_In,
    ExpressionBody_In_Await,
    ExpressionStatement,
    ExpressionStatement_Await,
    ExpressionStatement_Yield,
    ExpressionStatement_Yield_Await,
    Expression_Await,
    Expression_In,
    Expression_In_Await,
    Expression_In_Yield,
    Expression_In_Yield_Await,
    Expression_Yield,
    Expression_Yield_Await,
    FieldDefinition,
    FieldDefinition_Await,
    FieldDefinition_Yield,
    FieldDefinition_Yield_Await,
    Finally,
    Finally_Await,
    Finally_Await_Return,
    Finally_Return,
    Finally_Yield_Await_Return,
    Finally_Yield_Return,
    ForBinding,
    ForBinding_Await,
    ForBinding_Yield,
    ForBinding_Yield_Await,
    ForDeclaration,
    ForDeclaration_Await,
    ForDeclaration_Yield,
    ForDeclaration_Yield_Await,
    ForInOfStatement,
    ForInOfStatement_Await,
    ForInOfStatement_Await_Return,
    ForInOfStatement_Return,
    ForInOfStatement_Yield_Await_Return,
    ForInOfStatement_Yield_Return,
    ForStatement,
    ForStatement_Await,
    ForStatement_Await_Return,
    ForStatement_Return,
    ForStatement_Yield_Await_Return,
    ForStatement_Yield_Return,
    FormalParameter,
    FormalParameterList,
    FormalParameterList_Await,
    FormalParameterList_Yield,
    FormalParameterList_Yield_Await,
    FormalParameter_Await,
    FormalParameter_Yield,
    FormalParameter_Yield_Await,
    FormalParameters,
    FormalParameters_Await,
    FormalParameters_Yield,
    FormalParameters_Yield_Await,
    FromClause,
    FunctionBody,
    FunctionBody_Await,
    FunctionBody_Yield,
    FunctionBody_Yield_Await,
    FunctionDeclaration,
    FunctionDeclaration_Await,
    FunctionDeclaration_Await_Default,
    FunctionDeclaration_Yield,
    FunctionDeclaration_Yield_Await,
    FunctionExpression,
    FunctionRestParameter,
    FunctionRestParameter_Await,
    FunctionRestParameter_Yield,
    FunctionRestParameter_Yield_Await,
    FunctionStatementList,
    FunctionStatementList_Await,
    FunctionStatementList_Yield,
    FunctionStatementList_Yield_Await,
    GeneratorBody,
    GeneratorDeclaration,
    GeneratorDeclaration_Await,
    GeneratorDeclaration_Await_Default,
    GeneratorDeclaration_Yield,
    GeneratorDeclaration_Yield_Await,
    GeneratorExpression,
    GeneratorMethod,
    GeneratorMethod_Await,
    GeneratorMethod_Yield,
    GeneratorMethod_Yield_Await,
    HoistableDeclaration,
    HoistableDeclaration_Await,
    HoistableDeclaration_Await_Default,
    HoistableDeclaration_Yield,
    HoistableDeclaration_Yield_Await,
    Identifier,
    IdentifierNameButNotReservedWord,
    IdentifierReference,
    IdentifierReference_Await,
    IdentifierReference_Yield,
    IdentifierReference_Yield_Await,
    IfStatement,
    IfStatement_Await,
    IfStatement_Await_Return,
    IfStatement_Return,
    IfStatement_Yield_Await_Return,
    IfStatement_Yield_Return,
    ImportCall,
    ImportCall_Await,
    ImportCall_Yield,
    ImportCall_Yield_Await,
    ImportClause,
    ImportDeclaration,
    ImportMeta,
    ImportSpecifier,
    ImportedBinding,
    ImportedDefaultBinding,
    ImportsList,
    Initializer,
    Initializer_Await,
    Initializer_In,
    Initializer_In_Await,
    Initializer_In_Yield,
    Initializer_In_Yield_Await,
    Initializer_Yield,
    Initializer_Yield_Await,
    IterationStatement,
    IterationStatement_Await,
    IterationStatement_Await_Return,
    IterationStatement_Return,
    IterationStatement_Yield_Await_Return,
    IterationStatement_Yield_Return,
    KeywordOrIdentifierName,
    LabelIdentifier,
    LabelIdentifier_Await,
    LabelIdentifier_Yield,
    LabelIdentifier_Yield_Await,
    LabelledItem,
    LabelledItem_Await,
    LabelledItem_Await_Return,
    LabelledItem_Return,
    LabelledItem_Yield_Await_Return,
    LabelledItem_Yield_Return,
    LabelledStatement,
    LabelledStatement_Await,
    LabelledStatement_Await_Return,
    LabelledStatement_Return,
    LabelledStatement_Yield_Await_Return,
    LabelledStatement_Yield_Return,
    LeftHandSideExpression,
    LeftHandSideExpression_Await,
    LeftHandSideExpression_Yield,
    LeftHandSideExpression_Yield_Await,
    LexicalBinding,
    LexicalBinding_Await,
    LexicalBinding_In,
    LexicalBinding_In_Await,
    LexicalBinding_In_Yield,
    LexicalBinding_In_Yield_Await,
    LexicalBinding_Yield,
    LexicalBinding_Yield_Await,
    LexicalDeclaration,
    LexicalDeclaration_Await,
    LexicalDeclaration_In,
    LexicalDeclaration_In_Await,
    LexicalDeclaration_In_Yield,
    LexicalDeclaration_In_Yield_Await,
    LexicalDeclaration_Yield,
    LexicalDeclaration_Yield_Await,
    Literal,
    LiteralPropertyName,
    LogicalANDExpression,
    LogicalANDExpression_Await,
    LogicalANDExpression_In,
    LogicalANDExpression_In_Await,
    LogicalANDExpression_In_Yield,
    LogicalANDExpression_In_Yield_Await,
    LogicalANDExpression_Yield,
    LogicalANDExpression_Yield_Await,
    LogicalORExpression,
    LogicalORExpression_Await,
    LogicalORExpression_In,
    LogicalORExpression_In_Await,
    LogicalORExpression_In_Yield,
    LogicalORExpression_In_Yield_Await,
    LogicalORExpression_Yield,
    LogicalORExpression_Yield_Await,
    MemberExpression,
    MemberExpression_Await,
    MemberExpression_Yield,
    MemberExpression_Yield_Await,
    MetaProperty,
    MethodDefinition,
    MethodDefinition_Await,
    MethodDefinition_Yield,
    MethodDefinition_Yield_Await,
    Module,
    ModuleBody,
    ModuleExportName,
    ModuleItem,
    ModuleItemList,
    ModuleSpecifier,
    MultiplicativeExpression,
    MultiplicativeExpression_Await,
    MultiplicativeExpression_Yield,
    MultiplicativeExpression_Yield_Await,
    NameSpaceImport,
    NamedExports,
    NamedImports,
    NewExpression,
    NewExpression_Await,
    NewExpression_Yield,
    NewExpression_Yield_Await,
    NewTarget,
    NullLiteral,
    ObjectBindingPattern,
    ObjectBindingPattern_Await,
    ObjectBindingPattern_Yield,
    ObjectBindingPattern_Yield_Await,
    ObjectLiteral,
    ObjectLiteral_Await,
    ObjectLiteral_Yield,
    ObjectLiteral_Yield_Await,
    OptionalChain,
    OptionalChain_Await,
    OptionalChain_Yield,
    OptionalChain_Yield_Await,
    OptionalExpression,
    OptionalExpression_Await,
    OptionalExpression_Yield,
    OptionalExpression_Yield_Await,
    PrimaryExpression,
    PrimaryExpression_Await,
    PrimaryExpression_Yield,
    PrimaryExpression_Yield_Await,
    PropertyDefinition,
    PropertyDefinitionList,
    PropertyDefinitionList_Await,
    PropertyDefinitionList_Yield,
    PropertyDefinitionList_Yield_Await,
    PropertyDefinition_Await,
    PropertyDefinition_Yield,
    PropertyDefinition_Yield_Await,
    PropertyName,
    PropertyName_Await,
    PropertyName_Yield,
    PropertyName_Yield_Await,
    PropertySetParameterList,
    RelationalExpression,
    RelationalExpression_Await,
    RelationalExpression_In,
    RelationalExpression_In_Await,
    RelationalExpression_In_Yield,
    RelationalExpression_In_Yield_Await,
    RelationalExpression_Yield,
    RelationalExpression_Yield_Await,
    ReturnStatement,
    ReturnStatement_Await,
    ReturnStatement_Yield,
    ReturnStatement_Yield_Await,
    Script,
    ScriptBody,
    ShiftExpression,
    ShiftExpression_Await,
    ShiftExpression_Yield,
    ShiftExpression_Yield_Await,
    ShortCircuitExpression,
    ShortCircuitExpression_Await,
    ShortCircuitExpression_In,
    ShortCircuitExpression_In_Await,
    ShortCircuitExpression_In_Yield,
    ShortCircuitExpression_In_Yield_Await,
    ShortCircuitExpression_Yield,
    ShortCircuitExpression_Yield_Await,
    SingleNameBinding,
    SingleNameBinding_Await,
    SingleNameBinding_Yield,
    SingleNameBinding_Yield_Await,
    SpreadElement,
    SpreadElement_Await,
    SpreadElement_Yield,
    SpreadElement_Yield_Await,
    Statement,
    StatementList,
    StatementListItem,
    StatementListItem_Await,
    StatementListItem_Await_Return,
    StatementListItem_Return,
    StatementListItem_Yield_Await_Return,
    StatementListItem_Yield_Return,
    StatementList_Await,
    StatementList_Await_Return,
    StatementList_Return,
    StatementList_Yield_Await_Return,
    StatementList_Yield_Return,
    Statement_Await,
    Statement_Await_Return,
    Statement_Return,
    Statement_Yield_Await_Return,
    Statement_Yield_Return,
    SubstitutionTemplate,
    SubstitutionTemplate_Await,
    SubstitutionTemplate_Await_Tagged,
    SubstitutionTemplate_Tagged,
    SubstitutionTemplate_Yield,
    SubstitutionTemplate_Yield_Await,
    SubstitutionTemplate_Yield_Await_Tagged,
    SubstitutionTemplate_Yield_Tagged,
    SuperCall,
    SuperCall_Await,
    SuperCall_Yield,
    SuperCall_Yield_Await,
    SuperProperty,
    SuperProperty_Await,
    SuperProperty_Yield,
    SuperProperty_Yield_Await,
    SwitchStatement,
    SwitchStatement_Await,
    SwitchStatement_Await_Return,
    SwitchStatement_Return,
    SwitchStatement_Yield_Await_Return,
    SwitchStatement_Yield_Return,
    TemplateLiteral,
    TemplateLiteral_Await,
    TemplateLiteral_Await_Tagged,
    TemplateLiteral_Tagged,
    TemplateLiteral_Yield,
    TemplateLiteral_Yield_Await,
    TemplateLiteral_Yield_Await_Tagged,
    TemplateLiteral_Yield_Tagged,
    TemplateMiddleList,
    TemplateMiddleList_Await,
    TemplateMiddleList_Await_Tagged,
    TemplateMiddleList_Tagged,
    TemplateMiddleList_Yield,
    TemplateMiddleList_Yield_Await,
    TemplateMiddleList_Yield_Await_Tagged,
    TemplateMiddleList_Yield_Tagged,
    TemplateSpans,
    TemplateSpans_Await,
    TemplateSpans_Await_Tagged,
    TemplateSpans_Tagged,
    TemplateSpans_Yield,
    TemplateSpans_Yield_Await,
    TemplateSpans_Yield_Await_Tagged,
    TemplateSpans_Yield_Tagged,
    ThrowStatement,
    ThrowStatement_Await,
    ThrowStatement_Yield,
    ThrowStatement_Yield_Await,
    TryStatement,
    TryStatement_Await,
    TryStatement_Await_Return,
    TryStatement_Return,
    TryStatement_Yield_Await_Return,
    TryStatement_Yield_Return,
    UnaryExpression,
    UnaryExpression_Await,
    UnaryExpression_Yield,
    UnaryExpression_Yield_Await,
    UniqueFormalParameters,
    UniqueFormalParameters_Await,
    UniqueFormalParameters_Yield,
    UniqueFormalParameters_Yield_Await,
    UpdateExpression,
    UpdateExpression_Await,
    UpdateExpression_Yield,
    UpdateExpression_Yield_Await,
    VariableDeclaration,
    VariableDeclarationList,
    VariableDeclarationList_Await,
    VariableDeclarationList_In,
    VariableDeclarationList_In_Await,
    VariableDeclarationList_In_Yield,
    VariableDeclarationList_In_Yield_Await,
    VariableDeclarationList_Yield,
    VariableDeclarationList_Yield_Await,
    VariableDeclaration_Await,
    VariableDeclaration_In,
    VariableDeclaration_In_Await,
    VariableDeclaration_In_Yield,
    VariableDeclaration_In_Yield_Await,
    VariableDeclaration_Yield,
    VariableDeclaration_Yield_Await,
    VariableStatement,
    VariableStatement_Await,
    VariableStatement_Yield,
    VariableStatement_Yield_Await,
    WhileStatement,
    WhileStatement_Await,
    WhileStatement_Await_Return,
    WhileStatement_Return,
    WhileStatement_Yield_Await_Return,
    WhileStatement_Yield_Return,
    WithStatement,
    WithStatement_Await,
    WithStatement_Await_Return,
    WithStatement_Return,
    WithStatement_Yield_Await_Return,
    WithStatement_Yield_Return,
    YieldExpression,
    YieldExpression_Await,
    YieldExpression_In,
    YieldExpression_In_Await,
    _BLOCK_SCOPE_,
    _ELSE_BLOCK_,
    _FALSY_SHORT_CIRCUIT_,
    _FALSY_SHORT_CIRCUIT_ASSIGNMENT_,
    _FUNCTION_CONTEXT_,
    _FUNCTION_SIGNATURE_,
    _LOOP_INIT_,
    _LOOP_NEXT_,
    _LOOP_START_,
    _LOOP_TEST_,
    _NULLISH_SHORT_CIRCUIT_,
    _NULLISH_SHORT_CIRCUIT_ASSIGNMENT_,
    _THEN_BLOCK_,
    _TRUTHY_SHORT_CIRCUIT_,
    _TRUTHY_SHORT_CIRCUIT_ASSIGNMENT_,
}
