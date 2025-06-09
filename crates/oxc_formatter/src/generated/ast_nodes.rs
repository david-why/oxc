// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/generators/formatter2.rs`.

#![expect(clippy::match_same_arms, clippy::elidable_lifetime_names)]
use std::{mem::transmute, ops::Deref};

use oxc_allocator::{Allocator, Vec};
use oxc_ast::{AstKind, ast::*};
use oxc_span::GetSpan;

use crate::{
    formatter::{
        Buffer, Format, FormatResult, Formatter,
        trivia::{format_leading_comments, format_trailing_comments},
    },
    parentheses::NeedsParentheses,
    write::FormatWrite,
};

#[inline]
fn transmute_self<'a, T>(s: &AstNode<'a, T>) -> &'a AstNode<'a, T> {
    /// * SAFETY: `s` is already allocated in Arena, so transmute from `&` to `&'a` is safe.
    unsafe {
        transmute(s)
    }
}

pub enum AstNodes<'a> {
    Dummy(),
    Program(&'a AstNode<'a, Program<'a>>),
    IdentifierName(&'a AstNode<'a, IdentifierName<'a>>),
    IdentifierReference(&'a AstNode<'a, IdentifierReference<'a>>),
    BindingIdentifier(&'a AstNode<'a, BindingIdentifier<'a>>),
    LabelIdentifier(&'a AstNode<'a, LabelIdentifier<'a>>),
    ThisExpression(&'a AstNode<'a, ThisExpression>),
    ArrayExpression(&'a AstNode<'a, ArrayExpression<'a>>),
    ArrayExpressionElement(&'a AstNode<'a, ArrayExpressionElement<'a>>),
    Elision(&'a AstNode<'a, Elision>),
    ObjectExpression(&'a AstNode<'a, ObjectExpression<'a>>),
    ObjectProperty(&'a AstNode<'a, ObjectProperty<'a>>),
    PropertyKey(&'a AstNode<'a, PropertyKey<'a>>),
    TemplateLiteral(&'a AstNode<'a, TemplateLiteral<'a>>),
    TaggedTemplateExpression(&'a AstNode<'a, TaggedTemplateExpression<'a>>),
    MemberExpression(&'a AstNode<'a, MemberExpression<'a>>),
    CallExpression(&'a AstNode<'a, CallExpression<'a>>),
    NewExpression(&'a AstNode<'a, NewExpression<'a>>),
    MetaProperty(&'a AstNode<'a, MetaProperty<'a>>),
    SpreadElement(&'a AstNode<'a, SpreadElement<'a>>),
    Argument(&'a AstNode<'a, Argument<'a>>),
    UpdateExpression(&'a AstNode<'a, UpdateExpression<'a>>),
    UnaryExpression(&'a AstNode<'a, UnaryExpression<'a>>),
    BinaryExpression(&'a AstNode<'a, BinaryExpression<'a>>),
    PrivateInExpression(&'a AstNode<'a, PrivateInExpression<'a>>),
    LogicalExpression(&'a AstNode<'a, LogicalExpression<'a>>),
    ConditionalExpression(&'a AstNode<'a, ConditionalExpression<'a>>),
    AssignmentExpression(&'a AstNode<'a, AssignmentExpression<'a>>),
    AssignmentTarget(&'a AstNode<'a, AssignmentTarget<'a>>),
    SimpleAssignmentTarget(&'a AstNode<'a, SimpleAssignmentTarget<'a>>),
    AssignmentTargetPattern(&'a AstNode<'a, AssignmentTargetPattern<'a>>),
    ArrayAssignmentTarget(&'a AstNode<'a, ArrayAssignmentTarget<'a>>),
    ObjectAssignmentTarget(&'a AstNode<'a, ObjectAssignmentTarget<'a>>),
    AssignmentTargetWithDefault(&'a AstNode<'a, AssignmentTargetWithDefault<'a>>),
    SequenceExpression(&'a AstNode<'a, SequenceExpression<'a>>),
    Super(&'a AstNode<'a, Super>),
    AwaitExpression(&'a AstNode<'a, AwaitExpression<'a>>),
    ChainExpression(&'a AstNode<'a, ChainExpression<'a>>),
    ParenthesizedExpression(&'a AstNode<'a, ParenthesizedExpression<'a>>),
    Directive(&'a AstNode<'a, Directive<'a>>),
    Hashbang(&'a AstNode<'a, Hashbang<'a>>),
    BlockStatement(&'a AstNode<'a, BlockStatement<'a>>),
    VariableDeclaration(&'a AstNode<'a, VariableDeclaration<'a>>),
    VariableDeclarator(&'a AstNode<'a, VariableDeclarator<'a>>),
    EmptyStatement(&'a AstNode<'a, EmptyStatement>),
    ExpressionStatement(&'a AstNode<'a, ExpressionStatement<'a>>),
    IfStatement(&'a AstNode<'a, IfStatement<'a>>),
    DoWhileStatement(&'a AstNode<'a, DoWhileStatement<'a>>),
    WhileStatement(&'a AstNode<'a, WhileStatement<'a>>),
    ForStatement(&'a AstNode<'a, ForStatement<'a>>),
    ForStatementInit(&'a AstNode<'a, ForStatementInit<'a>>),
    ForInStatement(&'a AstNode<'a, ForInStatement<'a>>),
    ForOfStatement(&'a AstNode<'a, ForOfStatement<'a>>),
    ContinueStatement(&'a AstNode<'a, ContinueStatement<'a>>),
    BreakStatement(&'a AstNode<'a, BreakStatement<'a>>),
    ReturnStatement(&'a AstNode<'a, ReturnStatement<'a>>),
    WithStatement(&'a AstNode<'a, WithStatement<'a>>),
    SwitchStatement(&'a AstNode<'a, SwitchStatement<'a>>),
    SwitchCase(&'a AstNode<'a, SwitchCase<'a>>),
    LabeledStatement(&'a AstNode<'a, LabeledStatement<'a>>),
    ThrowStatement(&'a AstNode<'a, ThrowStatement<'a>>),
    TryStatement(&'a AstNode<'a, TryStatement<'a>>),
    CatchClause(&'a AstNode<'a, CatchClause<'a>>),
    CatchParameter(&'a AstNode<'a, CatchParameter<'a>>),
    DebuggerStatement(&'a AstNode<'a, DebuggerStatement>),
    AssignmentPattern(&'a AstNode<'a, AssignmentPattern<'a>>),
    ObjectPattern(&'a AstNode<'a, ObjectPattern<'a>>),
    ArrayPattern(&'a AstNode<'a, ArrayPattern<'a>>),
    BindingRestElement(&'a AstNode<'a, BindingRestElement<'a>>),
    Function(&'a AstNode<'a, Function<'a>>),
    FormalParameters(&'a AstNode<'a, FormalParameters<'a>>),
    FormalParameter(&'a AstNode<'a, FormalParameter<'a>>),
    FunctionBody(&'a AstNode<'a, FunctionBody<'a>>),
    ArrowFunctionExpression(&'a AstNode<'a, ArrowFunctionExpression<'a>>),
    YieldExpression(&'a AstNode<'a, YieldExpression<'a>>),
    Class(&'a AstNode<'a, Class<'a>>),
    ClassBody(&'a AstNode<'a, ClassBody<'a>>),
    MethodDefinition(&'a AstNode<'a, MethodDefinition<'a>>),
    PropertyDefinition(&'a AstNode<'a, PropertyDefinition<'a>>),
    PrivateIdentifier(&'a AstNode<'a, PrivateIdentifier<'a>>),
    StaticBlock(&'a AstNode<'a, StaticBlock<'a>>),
    ModuleDeclaration(&'a AstNode<'a, ModuleDeclaration<'a>>),
    ImportExpression(&'a AstNode<'a, ImportExpression<'a>>),
    ImportDeclaration(&'a AstNode<'a, ImportDeclaration<'a>>),
    ImportSpecifier(&'a AstNode<'a, ImportSpecifier<'a>>),
    ImportDefaultSpecifier(&'a AstNode<'a, ImportDefaultSpecifier<'a>>),
    ImportNamespaceSpecifier(&'a AstNode<'a, ImportNamespaceSpecifier<'a>>),
    ExportNamedDeclaration(&'a AstNode<'a, ExportNamedDeclaration<'a>>),
    ExportDefaultDeclaration(&'a AstNode<'a, ExportDefaultDeclaration<'a>>),
    ExportAllDeclaration(&'a AstNode<'a, ExportAllDeclaration<'a>>),
    ExportSpecifier(&'a AstNode<'a, ExportSpecifier<'a>>),
    V8IntrinsicExpression(&'a AstNode<'a, V8IntrinsicExpression<'a>>),
    BooleanLiteral(&'a AstNode<'a, BooleanLiteral>),
    NullLiteral(&'a AstNode<'a, NullLiteral>),
    NumericLiteral(&'a AstNode<'a, NumericLiteral<'a>>),
    StringLiteral(&'a AstNode<'a, StringLiteral<'a>>),
    BigIntLiteral(&'a AstNode<'a, BigIntLiteral<'a>>),
    RegExpLiteral(&'a AstNode<'a, RegExpLiteral<'a>>),
    JSXElement(&'a AstNode<'a, JSXElement<'a>>),
    JSXOpeningElement(&'a AstNode<'a, JSXOpeningElement<'a>>),
    JSXClosingElement(&'a AstNode<'a, JSXClosingElement<'a>>),
    JSXFragment(&'a AstNode<'a, JSXFragment<'a>>),
    JSXElementName(&'a AstNode<'a, JSXElementName<'a>>),
    JSXNamespacedName(&'a AstNode<'a, JSXNamespacedName<'a>>),
    JSXMemberExpression(&'a AstNode<'a, JSXMemberExpression<'a>>),
    JSXMemberExpressionObject(&'a AstNode<'a, JSXMemberExpressionObject<'a>>),
    JSXExpressionContainer(&'a AstNode<'a, JSXExpressionContainer<'a>>),
    JSXAttributeItem(&'a AstNode<'a, JSXAttributeItem<'a>>),
    JSXSpreadAttribute(&'a AstNode<'a, JSXSpreadAttribute<'a>>),
    JSXIdentifier(&'a AstNode<'a, JSXIdentifier<'a>>),
    JSXText(&'a AstNode<'a, JSXText<'a>>),
    TSThisParameter(&'a AstNode<'a, TSThisParameter<'a>>),
    TSEnumDeclaration(&'a AstNode<'a, TSEnumDeclaration<'a>>),
    TSEnumBody(&'a AstNode<'a, TSEnumBody<'a>>),
    TSEnumMember(&'a AstNode<'a, TSEnumMember<'a>>),
    TSTypeAnnotation(&'a AstNode<'a, TSTypeAnnotation<'a>>),
    TSLiteralType(&'a AstNode<'a, TSLiteralType<'a>>),
    TSConditionalType(&'a AstNode<'a, TSConditionalType<'a>>),
    TSUnionType(&'a AstNode<'a, TSUnionType<'a>>),
    TSIntersectionType(&'a AstNode<'a, TSIntersectionType<'a>>),
    TSParenthesizedType(&'a AstNode<'a, TSParenthesizedType<'a>>),
    TSIndexedAccessType(&'a AstNode<'a, TSIndexedAccessType<'a>>),
    TSNamedTupleMember(&'a AstNode<'a, TSNamedTupleMember<'a>>),
    TSAnyKeyword(&'a AstNode<'a, TSAnyKeyword>),
    TSStringKeyword(&'a AstNode<'a, TSStringKeyword>),
    TSBooleanKeyword(&'a AstNode<'a, TSBooleanKeyword>),
    TSNumberKeyword(&'a AstNode<'a, TSNumberKeyword>),
    TSNeverKeyword(&'a AstNode<'a, TSNeverKeyword>),
    TSIntrinsicKeyword(&'a AstNode<'a, TSIntrinsicKeyword>),
    TSUnknownKeyword(&'a AstNode<'a, TSUnknownKeyword>),
    TSNullKeyword(&'a AstNode<'a, TSNullKeyword>),
    TSUndefinedKeyword(&'a AstNode<'a, TSUndefinedKeyword>),
    TSVoidKeyword(&'a AstNode<'a, TSVoidKeyword>),
    TSSymbolKeyword(&'a AstNode<'a, TSSymbolKeyword>),
    TSThisType(&'a AstNode<'a, TSThisType>),
    TSObjectKeyword(&'a AstNode<'a, TSObjectKeyword>),
    TSBigIntKeyword(&'a AstNode<'a, TSBigIntKeyword>),
    TSTypeReference(&'a AstNode<'a, TSTypeReference<'a>>),
    TSTypeName(&'a AstNode<'a, TSTypeName<'a>>),
    TSQualifiedName(&'a AstNode<'a, TSQualifiedName<'a>>),
    TSTypeParameterInstantiation(&'a AstNode<'a, TSTypeParameterInstantiation<'a>>),
    TSTypeParameter(&'a AstNode<'a, TSTypeParameter<'a>>),
    TSTypeParameterDeclaration(&'a AstNode<'a, TSTypeParameterDeclaration<'a>>),
    TSTypeAliasDeclaration(&'a AstNode<'a, TSTypeAliasDeclaration<'a>>),
    TSClassImplements(&'a AstNode<'a, TSClassImplements<'a>>),
    TSInterfaceDeclaration(&'a AstNode<'a, TSInterfaceDeclaration<'a>>),
    TSPropertySignature(&'a AstNode<'a, TSPropertySignature<'a>>),
    TSMethodSignature(&'a AstNode<'a, TSMethodSignature<'a>>),
    TSConstructSignatureDeclaration(&'a AstNode<'a, TSConstructSignatureDeclaration<'a>>),
    TSInterfaceHeritage(&'a AstNode<'a, TSInterfaceHeritage<'a>>),
    TSModuleDeclaration(&'a AstNode<'a, TSModuleDeclaration<'a>>),
    TSModuleBlock(&'a AstNode<'a, TSModuleBlock<'a>>),
    TSTypeLiteral(&'a AstNode<'a, TSTypeLiteral<'a>>),
    TSInferType(&'a AstNode<'a, TSInferType<'a>>),
    TSTypeQuery(&'a AstNode<'a, TSTypeQuery<'a>>),
    TSImportType(&'a AstNode<'a, TSImportType<'a>>),
    TSMappedType(&'a AstNode<'a, TSMappedType<'a>>),
    TSTemplateLiteralType(&'a AstNode<'a, TSTemplateLiteralType<'a>>),
    TSAsExpression(&'a AstNode<'a, TSAsExpression<'a>>),
    TSSatisfiesExpression(&'a AstNode<'a, TSSatisfiesExpression<'a>>),
    TSTypeAssertion(&'a AstNode<'a, TSTypeAssertion<'a>>),
    TSImportEqualsDeclaration(&'a AstNode<'a, TSImportEqualsDeclaration<'a>>),
    TSModuleReference(&'a AstNode<'a, TSModuleReference<'a>>),
    TSExternalModuleReference(&'a AstNode<'a, TSExternalModuleReference<'a>>),
    TSNonNullExpression(&'a AstNode<'a, TSNonNullExpression<'a>>),
    Decorator(&'a AstNode<'a, Decorator<'a>>),
    TSExportAssignment(&'a AstNode<'a, TSExportAssignment<'a>>),
    TSInstantiationExpression(&'a AstNode<'a, TSInstantiationExpression<'a>>),
}
impl<'a> AstNodes<'a> {
    #[inline]
    pub fn span(&self) -> Span {
        match self {
            Self::Dummy() => panic!("Should never be called on a dummy node"),
            Self::Program(n) => n.span(),
            Self::IdentifierName(n) => n.span(),
            Self::IdentifierReference(n) => n.span(),
            Self::BindingIdentifier(n) => n.span(),
            Self::LabelIdentifier(n) => n.span(),
            Self::ThisExpression(n) => n.span(),
            Self::ArrayExpression(n) => n.span(),
            Self::ArrayExpressionElement(n) => n.span(),
            Self::Elision(n) => n.span(),
            Self::ObjectExpression(n) => n.span(),
            Self::ObjectProperty(n) => n.span(),
            Self::PropertyKey(n) => n.span(),
            Self::TemplateLiteral(n) => n.span(),
            Self::TaggedTemplateExpression(n) => n.span(),
            Self::MemberExpression(n) => n.span(),
            Self::CallExpression(n) => n.span(),
            Self::NewExpression(n) => n.span(),
            Self::MetaProperty(n) => n.span(),
            Self::SpreadElement(n) => n.span(),
            Self::Argument(n) => n.span(),
            Self::UpdateExpression(n) => n.span(),
            Self::UnaryExpression(n) => n.span(),
            Self::BinaryExpression(n) => n.span(),
            Self::PrivateInExpression(n) => n.span(),
            Self::LogicalExpression(n) => n.span(),
            Self::ConditionalExpression(n) => n.span(),
            Self::AssignmentExpression(n) => n.span(),
            Self::AssignmentTarget(n) => n.span(),
            Self::SimpleAssignmentTarget(n) => n.span(),
            Self::AssignmentTargetPattern(n) => n.span(),
            Self::ArrayAssignmentTarget(n) => n.span(),
            Self::ObjectAssignmentTarget(n) => n.span(),
            Self::AssignmentTargetWithDefault(n) => n.span(),
            Self::SequenceExpression(n) => n.span(),
            Self::Super(n) => n.span(),
            Self::AwaitExpression(n) => n.span(),
            Self::ChainExpression(n) => n.span(),
            Self::ParenthesizedExpression(n) => n.span(),
            Self::Directive(n) => n.span(),
            Self::Hashbang(n) => n.span(),
            Self::BlockStatement(n) => n.span(),
            Self::VariableDeclaration(n) => n.span(),
            Self::VariableDeclarator(n) => n.span(),
            Self::EmptyStatement(n) => n.span(),
            Self::ExpressionStatement(n) => n.span(),
            Self::IfStatement(n) => n.span(),
            Self::DoWhileStatement(n) => n.span(),
            Self::WhileStatement(n) => n.span(),
            Self::ForStatement(n) => n.span(),
            Self::ForStatementInit(n) => n.span(),
            Self::ForInStatement(n) => n.span(),
            Self::ForOfStatement(n) => n.span(),
            Self::ContinueStatement(n) => n.span(),
            Self::BreakStatement(n) => n.span(),
            Self::ReturnStatement(n) => n.span(),
            Self::WithStatement(n) => n.span(),
            Self::SwitchStatement(n) => n.span(),
            Self::SwitchCase(n) => n.span(),
            Self::LabeledStatement(n) => n.span(),
            Self::ThrowStatement(n) => n.span(),
            Self::TryStatement(n) => n.span(),
            Self::CatchClause(n) => n.span(),
            Self::CatchParameter(n) => n.span(),
            Self::DebuggerStatement(n) => n.span(),
            Self::AssignmentPattern(n) => n.span(),
            Self::ObjectPattern(n) => n.span(),
            Self::ArrayPattern(n) => n.span(),
            Self::BindingRestElement(n) => n.span(),
            Self::Function(n) => n.span(),
            Self::FormalParameters(n) => n.span(),
            Self::FormalParameter(n) => n.span(),
            Self::FunctionBody(n) => n.span(),
            Self::ArrowFunctionExpression(n) => n.span(),
            Self::YieldExpression(n) => n.span(),
            Self::Class(n) => n.span(),
            Self::ClassBody(n) => n.span(),
            Self::MethodDefinition(n) => n.span(),
            Self::PropertyDefinition(n) => n.span(),
            Self::PrivateIdentifier(n) => n.span(),
            Self::StaticBlock(n) => n.span(),
            Self::ModuleDeclaration(n) => n.span(),
            Self::ImportExpression(n) => n.span(),
            Self::ImportDeclaration(n) => n.span(),
            Self::ImportSpecifier(n) => n.span(),
            Self::ImportDefaultSpecifier(n) => n.span(),
            Self::ImportNamespaceSpecifier(n) => n.span(),
            Self::ExportNamedDeclaration(n) => n.span(),
            Self::ExportDefaultDeclaration(n) => n.span(),
            Self::ExportAllDeclaration(n) => n.span(),
            Self::ExportSpecifier(n) => n.span(),
            Self::V8IntrinsicExpression(n) => n.span(),
            Self::BooleanLiteral(n) => n.span(),
            Self::NullLiteral(n) => n.span(),
            Self::NumericLiteral(n) => n.span(),
            Self::StringLiteral(n) => n.span(),
            Self::BigIntLiteral(n) => n.span(),
            Self::RegExpLiteral(n) => n.span(),
            Self::JSXElement(n) => n.span(),
            Self::JSXOpeningElement(n) => n.span(),
            Self::JSXClosingElement(n) => n.span(),
            Self::JSXFragment(n) => n.span(),
            Self::JSXElementName(n) => n.span(),
            Self::JSXNamespacedName(n) => n.span(),
            Self::JSXMemberExpression(n) => n.span(),
            Self::JSXMemberExpressionObject(n) => n.span(),
            Self::JSXExpressionContainer(n) => n.span(),
            Self::JSXAttributeItem(n) => n.span(),
            Self::JSXSpreadAttribute(n) => n.span(),
            Self::JSXIdentifier(n) => n.span(),
            Self::JSXText(n) => n.span(),
            Self::TSThisParameter(n) => n.span(),
            Self::TSEnumDeclaration(n) => n.span(),
            Self::TSEnumBody(n) => n.span(),
            Self::TSEnumMember(n) => n.span(),
            Self::TSTypeAnnotation(n) => n.span(),
            Self::TSLiteralType(n) => n.span(),
            Self::TSConditionalType(n) => n.span(),
            Self::TSUnionType(n) => n.span(),
            Self::TSIntersectionType(n) => n.span(),
            Self::TSParenthesizedType(n) => n.span(),
            Self::TSIndexedAccessType(n) => n.span(),
            Self::TSNamedTupleMember(n) => n.span(),
            Self::TSAnyKeyword(n) => n.span(),
            Self::TSStringKeyword(n) => n.span(),
            Self::TSBooleanKeyword(n) => n.span(),
            Self::TSNumberKeyword(n) => n.span(),
            Self::TSNeverKeyword(n) => n.span(),
            Self::TSIntrinsicKeyword(n) => n.span(),
            Self::TSUnknownKeyword(n) => n.span(),
            Self::TSNullKeyword(n) => n.span(),
            Self::TSUndefinedKeyword(n) => n.span(),
            Self::TSVoidKeyword(n) => n.span(),
            Self::TSSymbolKeyword(n) => n.span(),
            Self::TSThisType(n) => n.span(),
            Self::TSObjectKeyword(n) => n.span(),
            Self::TSBigIntKeyword(n) => n.span(),
            Self::TSTypeReference(n) => n.span(),
            Self::TSTypeName(n) => n.span(),
            Self::TSQualifiedName(n) => n.span(),
            Self::TSTypeParameterInstantiation(n) => n.span(),
            Self::TSTypeParameter(n) => n.span(),
            Self::TSTypeParameterDeclaration(n) => n.span(),
            Self::TSTypeAliasDeclaration(n) => n.span(),
            Self::TSClassImplements(n) => n.span(),
            Self::TSInterfaceDeclaration(n) => n.span(),
            Self::TSPropertySignature(n) => n.span(),
            Self::TSMethodSignature(n) => n.span(),
            Self::TSConstructSignatureDeclaration(n) => n.span(),
            Self::TSInterfaceHeritage(n) => n.span(),
            Self::TSModuleDeclaration(n) => n.span(),
            Self::TSModuleBlock(n) => n.span(),
            Self::TSTypeLiteral(n) => n.span(),
            Self::TSInferType(n) => n.span(),
            Self::TSTypeQuery(n) => n.span(),
            Self::TSImportType(n) => n.span(),
            Self::TSMappedType(n) => n.span(),
            Self::TSTemplateLiteralType(n) => n.span(),
            Self::TSAsExpression(n) => n.span(),
            Self::TSSatisfiesExpression(n) => n.span(),
            Self::TSTypeAssertion(n) => n.span(),
            Self::TSImportEqualsDeclaration(n) => n.span(),
            Self::TSModuleReference(n) => n.span(),
            Self::TSExternalModuleReference(n) => n.span(),
            Self::TSNonNullExpression(n) => n.span(),
            Self::Decorator(n) => n.span(),
            Self::TSExportAssignment(n) => n.span(),
            Self::TSInstantiationExpression(n) => n.span(),
        }
    }
    #[inline]
    pub fn parent(&self) -> &'a Self {
        match self {
            Self::Dummy() => panic!("Should never be called on a dummy node"),
            Self::Program(n) => n.parent,
            Self::IdentifierName(n) => n.parent,
            Self::IdentifierReference(n) => n.parent,
            Self::BindingIdentifier(n) => n.parent,
            Self::LabelIdentifier(n) => n.parent,
            Self::ThisExpression(n) => n.parent,
            Self::ArrayExpression(n) => n.parent,
            Self::ArrayExpressionElement(n) => n.parent,
            Self::Elision(n) => n.parent,
            Self::ObjectExpression(n) => n.parent,
            Self::ObjectProperty(n) => n.parent,
            Self::PropertyKey(n) => n.parent,
            Self::TemplateLiteral(n) => n.parent,
            Self::TaggedTemplateExpression(n) => n.parent,
            Self::MemberExpression(n) => n.parent,
            Self::CallExpression(n) => n.parent,
            Self::NewExpression(n) => n.parent,
            Self::MetaProperty(n) => n.parent,
            Self::SpreadElement(n) => n.parent,
            Self::Argument(n) => n.parent,
            Self::UpdateExpression(n) => n.parent,
            Self::UnaryExpression(n) => n.parent,
            Self::BinaryExpression(n) => n.parent,
            Self::PrivateInExpression(n) => n.parent,
            Self::LogicalExpression(n) => n.parent,
            Self::ConditionalExpression(n) => n.parent,
            Self::AssignmentExpression(n) => n.parent,
            Self::AssignmentTarget(n) => n.parent,
            Self::SimpleAssignmentTarget(n) => n.parent,
            Self::AssignmentTargetPattern(n) => n.parent,
            Self::ArrayAssignmentTarget(n) => n.parent,
            Self::ObjectAssignmentTarget(n) => n.parent,
            Self::AssignmentTargetWithDefault(n) => n.parent,
            Self::SequenceExpression(n) => n.parent,
            Self::Super(n) => n.parent,
            Self::AwaitExpression(n) => n.parent,
            Self::ChainExpression(n) => n.parent,
            Self::ParenthesizedExpression(n) => n.parent,
            Self::Directive(n) => n.parent,
            Self::Hashbang(n) => n.parent,
            Self::BlockStatement(n) => n.parent,
            Self::VariableDeclaration(n) => n.parent,
            Self::VariableDeclarator(n) => n.parent,
            Self::EmptyStatement(n) => n.parent,
            Self::ExpressionStatement(n) => n.parent,
            Self::IfStatement(n) => n.parent,
            Self::DoWhileStatement(n) => n.parent,
            Self::WhileStatement(n) => n.parent,
            Self::ForStatement(n) => n.parent,
            Self::ForStatementInit(n) => n.parent,
            Self::ForInStatement(n) => n.parent,
            Self::ForOfStatement(n) => n.parent,
            Self::ContinueStatement(n) => n.parent,
            Self::BreakStatement(n) => n.parent,
            Self::ReturnStatement(n) => n.parent,
            Self::WithStatement(n) => n.parent,
            Self::SwitchStatement(n) => n.parent,
            Self::SwitchCase(n) => n.parent,
            Self::LabeledStatement(n) => n.parent,
            Self::ThrowStatement(n) => n.parent,
            Self::TryStatement(n) => n.parent,
            Self::CatchClause(n) => n.parent,
            Self::CatchParameter(n) => n.parent,
            Self::DebuggerStatement(n) => n.parent,
            Self::AssignmentPattern(n) => n.parent,
            Self::ObjectPattern(n) => n.parent,
            Self::ArrayPattern(n) => n.parent,
            Self::BindingRestElement(n) => n.parent,
            Self::Function(n) => n.parent,
            Self::FormalParameters(n) => n.parent,
            Self::FormalParameter(n) => n.parent,
            Self::FunctionBody(n) => n.parent,
            Self::ArrowFunctionExpression(n) => n.parent,
            Self::YieldExpression(n) => n.parent,
            Self::Class(n) => n.parent,
            Self::ClassBody(n) => n.parent,
            Self::MethodDefinition(n) => n.parent,
            Self::PropertyDefinition(n) => n.parent,
            Self::PrivateIdentifier(n) => n.parent,
            Self::StaticBlock(n) => n.parent,
            Self::ModuleDeclaration(n) => n.parent,
            Self::ImportExpression(n) => n.parent,
            Self::ImportDeclaration(n) => n.parent,
            Self::ImportSpecifier(n) => n.parent,
            Self::ImportDefaultSpecifier(n) => n.parent,
            Self::ImportNamespaceSpecifier(n) => n.parent,
            Self::ExportNamedDeclaration(n) => n.parent,
            Self::ExportDefaultDeclaration(n) => n.parent,
            Self::ExportAllDeclaration(n) => n.parent,
            Self::ExportSpecifier(n) => n.parent,
            Self::V8IntrinsicExpression(n) => n.parent,
            Self::BooleanLiteral(n) => n.parent,
            Self::NullLiteral(n) => n.parent,
            Self::NumericLiteral(n) => n.parent,
            Self::StringLiteral(n) => n.parent,
            Self::BigIntLiteral(n) => n.parent,
            Self::RegExpLiteral(n) => n.parent,
            Self::JSXElement(n) => n.parent,
            Self::JSXOpeningElement(n) => n.parent,
            Self::JSXClosingElement(n) => n.parent,
            Self::JSXFragment(n) => n.parent,
            Self::JSXElementName(n) => n.parent,
            Self::JSXNamespacedName(n) => n.parent,
            Self::JSXMemberExpression(n) => n.parent,
            Self::JSXMemberExpressionObject(n) => n.parent,
            Self::JSXExpressionContainer(n) => n.parent,
            Self::JSXAttributeItem(n) => n.parent,
            Self::JSXSpreadAttribute(n) => n.parent,
            Self::JSXIdentifier(n) => n.parent,
            Self::JSXText(n) => n.parent,
            Self::TSThisParameter(n) => n.parent,
            Self::TSEnumDeclaration(n) => n.parent,
            Self::TSEnumBody(n) => n.parent,
            Self::TSEnumMember(n) => n.parent,
            Self::TSTypeAnnotation(n) => n.parent,
            Self::TSLiteralType(n) => n.parent,
            Self::TSConditionalType(n) => n.parent,
            Self::TSUnionType(n) => n.parent,
            Self::TSIntersectionType(n) => n.parent,
            Self::TSParenthesizedType(n) => n.parent,
            Self::TSIndexedAccessType(n) => n.parent,
            Self::TSNamedTupleMember(n) => n.parent,
            Self::TSAnyKeyword(n) => n.parent,
            Self::TSStringKeyword(n) => n.parent,
            Self::TSBooleanKeyword(n) => n.parent,
            Self::TSNumberKeyword(n) => n.parent,
            Self::TSNeverKeyword(n) => n.parent,
            Self::TSIntrinsicKeyword(n) => n.parent,
            Self::TSUnknownKeyword(n) => n.parent,
            Self::TSNullKeyword(n) => n.parent,
            Self::TSUndefinedKeyword(n) => n.parent,
            Self::TSVoidKeyword(n) => n.parent,
            Self::TSSymbolKeyword(n) => n.parent,
            Self::TSThisType(n) => n.parent,
            Self::TSObjectKeyword(n) => n.parent,
            Self::TSBigIntKeyword(n) => n.parent,
            Self::TSTypeReference(n) => n.parent,
            Self::TSTypeName(n) => n.parent,
            Self::TSQualifiedName(n) => n.parent,
            Self::TSTypeParameterInstantiation(n) => n.parent,
            Self::TSTypeParameter(n) => n.parent,
            Self::TSTypeParameterDeclaration(n) => n.parent,
            Self::TSTypeAliasDeclaration(n) => n.parent,
            Self::TSClassImplements(n) => n.parent,
            Self::TSInterfaceDeclaration(n) => n.parent,
            Self::TSPropertySignature(n) => n.parent,
            Self::TSMethodSignature(n) => n.parent,
            Self::TSConstructSignatureDeclaration(n) => n.parent,
            Self::TSInterfaceHeritage(n) => n.parent,
            Self::TSModuleDeclaration(n) => n.parent,
            Self::TSModuleBlock(n) => n.parent,
            Self::TSTypeLiteral(n) => n.parent,
            Self::TSInferType(n) => n.parent,
            Self::TSTypeQuery(n) => n.parent,
            Self::TSImportType(n) => n.parent,
            Self::TSMappedType(n) => n.parent,
            Self::TSTemplateLiteralType(n) => n.parent,
            Self::TSAsExpression(n) => n.parent,
            Self::TSSatisfiesExpression(n) => n.parent,
            Self::TSTypeAssertion(n) => n.parent,
            Self::TSImportEqualsDeclaration(n) => n.parent,
            Self::TSModuleReference(n) => n.parent,
            Self::TSExternalModuleReference(n) => n.parent,
            Self::TSNonNullExpression(n) => n.parent,
            Self::Decorator(n) => n.parent,
            Self::TSExportAssignment(n) => n.parent,
            Self::TSInstantiationExpression(n) => n.parent,
        }
    }
}

pub struct AstNode<'a, T> {
    inner: &'a T,
    pub parent: &'a AstNodes<'a>,
    allocator: &'a Allocator,
}

impl<'a, T> Deref for AstNode<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}
impl<'a, T> AsRef<T> for AstNode<'a, T> {
    fn as_ref(&self) -> &T {
        self.inner
    }
}

impl<'a> AstNode<'a, Program<'a>> {
    pub fn new(inner: &'a Program<'a>, parent: &'a AstNodes<'a>, allocator: &'a Allocator) -> Self {
        AstNode { inner, parent, allocator }
    }
}

impl<'a, T> AstNode<'a, Option<T>> {
    pub fn as_ref(&self) -> Option<&'a AstNode<'a, T>> {
        self.allocator
            .alloc(self.inner.as_ref().map(|inner| AstNode {
                inner,
                parent: self.parent,
                allocator: self.allocator,
            }))
            .as_ref()
    }
}

impl<'a, T> AstNode<'a, Vec<'a, T>> {
    pub fn iter(&self) -> AstNodeIterator<'a, T> {
        AstNodeIterator { inner: self.inner.iter(), parent: self.parent, allocator: self.allocator }
    }

    pub fn first(&self) -> Option<&'a AstNode<'a, T>> {
        self.allocator
            .alloc(self.inner.first().map(|inner| AstNode {
                inner,
                parent: self.parent,
                allocator: self.allocator,
            }))
            .as_ref()
    }

    pub fn last(&self) -> Option<&'a AstNode<'a, T>> {
        self.allocator
            .alloc(self.inner.last().map(|inner| AstNode {
                inner,
                parent: self.parent,
                allocator: self.allocator,
            }))
            .as_ref()
    }
}

pub struct AstNodeIterator<'a, T> {
    inner: std::slice::Iter<'a, T>,
    parent: &'a AstNodes<'a>,
    allocator: &'a Allocator,
}

impl<'a, T> Iterator for AstNodeIterator<'a, T> {
    type Item = &'a AstNode<'a, T>;
    fn next(&mut self) -> Option<Self::Item> {
        let allocator = self.allocator;
        allocator
            .alloc(self.inner.next().map(|inner| AstNode { parent: self.parent, inner, allocator }))
            .as_ref()
    }
}

impl<'a, T> IntoIterator for &AstNode<'a, Vec<'a, T>> {
    type Item = &'a AstNode<'a, T>;
    type IntoIter = AstNodeIterator<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        AstNodeIterator::<T> {
            inner: self.inner.iter(),
            parent: self.parent,
            allocator: self.allocator,
        }
    }
}

impl<'a> AstNode<'a, Program<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn source_type(&self) -> SourceType {
        self.inner.source_type
    }

    #[inline]
    pub fn source_text(&self) -> &'a str {
        self.inner.source_text
    }

    #[inline]
    pub fn comments(&self) -> &AstNode<'a, Vec<'a, Comment>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.comments,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::Program(transmute_self(self))),
        })
    }

    #[inline]
    pub fn hashbang(&self) -> Option<&AstNode<'a, Hashbang<'a>>> {
        self.allocator
            .alloc(self.inner.hashbang.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::Program(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn directives(&self) -> &AstNode<'a, Vec<'a, Directive<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.directives,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::Program(transmute_self(self))),
        })
    }

    #[inline]
    pub fn body(&self) -> &AstNode<'a, Vec<'a, Statement<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.body,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::Program(transmute_self(self))),
        })
    }
}

impl<'a> AstNode<'a, Expression<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.parent;
        let node = match self.inner {
            Expression::BooleanLiteral(s) => {
                AstNodes::BooleanLiteral(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::NullLiteral(s) => AstNodes::NullLiteral(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            Expression::NumericLiteral(s) => {
                AstNodes::NumericLiteral(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::BigIntLiteral(s) => {
                AstNodes::BigIntLiteral(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::RegExpLiteral(s) => {
                AstNodes::RegExpLiteral(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::StringLiteral(s) => {
                AstNodes::StringLiteral(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::TemplateLiteral(s) => {
                AstNodes::TemplateLiteral(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::Identifier(s) => {
                AstNodes::IdentifierReference(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::MetaProperty(s) => AstNodes::MetaProperty(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            Expression::Super(s) => AstNodes::Super(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            Expression::ArrayExpression(s) => {
                AstNodes::ArrayExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::ArrowFunctionExpression(s) => {
                AstNodes::ArrowFunctionExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::AssignmentExpression(s) => {
                AstNodes::AssignmentExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::AwaitExpression(s) => {
                AstNodes::AwaitExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::BinaryExpression(s) => {
                AstNodes::BinaryExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::CallExpression(s) => {
                AstNodes::CallExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::ChainExpression(s) => {
                AstNodes::ChainExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::ClassExpression(s) => AstNodes::Class(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            Expression::ConditionalExpression(s) => {
                AstNodes::ConditionalExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::FunctionExpression(s) => {
                AstNodes::Function(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::ImportExpression(s) => {
                AstNodes::ImportExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::LogicalExpression(s) => {
                AstNodes::LogicalExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::NewExpression(s) => {
                AstNodes::NewExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::ObjectExpression(s) => {
                AstNodes::ObjectExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::ParenthesizedExpression(s) => {
                AstNodes::ParenthesizedExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::SequenceExpression(s) => {
                AstNodes::SequenceExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::TaggedTemplateExpression(s) => {
                AstNodes::TaggedTemplateExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::ThisExpression(s) => {
                AstNodes::ThisExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::UnaryExpression(s) => {
                AstNodes::UnaryExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::UpdateExpression(s) => {
                AstNodes::UpdateExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::YieldExpression(s) => {
                AstNodes::YieldExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::PrivateInExpression(s) => {
                AstNodes::PrivateInExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::JSXElement(s) => AstNodes::JSXElement(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            Expression::JSXFragment(s) => AstNodes::JSXFragment(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            Expression::TSAsExpression(s) => {
                AstNodes::TSAsExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::TSSatisfiesExpression(s) => {
                AstNodes::TSSatisfiesExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::TSTypeAssertion(s) => {
                AstNodes::TSTypeAssertion(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::TSNonNullExpression(s) => {
                AstNodes::TSNonNullExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::TSInstantiationExpression(s) => {
                AstNodes::TSInstantiationExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Expression::V8IntrinsicExpression(s) => {
                AstNodes::V8IntrinsicExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            it @ match_member_expression!(Expression) => {
                AstNodes::MemberExpression(self.allocator.alloc(AstNode {
                    inner: it.to_member_expression(),
                    parent,
                    allocator: self.allocator,
                }))
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, Expression<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.parent;
        match self.inner {
            Expression::BooleanLiteral(inner) => self
                .allocator
                .alloc(AstNode::<BooleanLiteral> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::NullLiteral(inner) => self
                .allocator
                .alloc(AstNode::<NullLiteral> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::NumericLiteral(inner) => self
                .allocator
                .alloc(AstNode::<NumericLiteral> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::BigIntLiteral(inner) => self
                .allocator
                .alloc(AstNode::<BigIntLiteral> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::RegExpLiteral(inner) => self
                .allocator
                .alloc(AstNode::<RegExpLiteral> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::StringLiteral(inner) => self
                .allocator
                .alloc(AstNode::<StringLiteral> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::TemplateLiteral(inner) => self
                .allocator
                .alloc(AstNode::<TemplateLiteral> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::Identifier(inner) => self
                .allocator
                .alloc(AstNode::<IdentifierReference> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::MetaProperty(inner) => self
                .allocator
                .alloc(AstNode::<MetaProperty> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::Super(inner) => self
                .allocator
                .alloc(AstNode::<Super> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::ArrayExpression(inner) => self
                .allocator
                .alloc(AstNode::<ArrayExpression> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::ArrowFunctionExpression(inner) => self
                .allocator
                .alloc(AstNode::<ArrowFunctionExpression> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
            Expression::AssignmentExpression(inner) => self
                .allocator
                .alloc(AstNode::<AssignmentExpression> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::AwaitExpression(inner) => self
                .allocator
                .alloc(AstNode::<AwaitExpression> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::BinaryExpression(inner) => self
                .allocator
                .alloc(AstNode::<BinaryExpression> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::CallExpression(inner) => self
                .allocator
                .alloc(AstNode::<CallExpression> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::ChainExpression(inner) => self
                .allocator
                .alloc(AstNode::<ChainExpression> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::ClassExpression(inner) => self
                .allocator
                .alloc(AstNode::<Class> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::ConditionalExpression(inner) => self
                .allocator
                .alloc(AstNode::<ConditionalExpression> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
            Expression::FunctionExpression(inner) => self
                .allocator
                .alloc(AstNode::<Function> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::ImportExpression(inner) => self
                .allocator
                .alloc(AstNode::<ImportExpression> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::LogicalExpression(inner) => self
                .allocator
                .alloc(AstNode::<LogicalExpression> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::NewExpression(inner) => self
                .allocator
                .alloc(AstNode::<NewExpression> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::ObjectExpression(inner) => self
                .allocator
                .alloc(AstNode::<ObjectExpression> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::ParenthesizedExpression(inner) => self
                .allocator
                .alloc(AstNode::<ParenthesizedExpression> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
            Expression::SequenceExpression(inner) => self
                .allocator
                .alloc(AstNode::<SequenceExpression> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::TaggedTemplateExpression(inner) => self
                .allocator
                .alloc(AstNode::<TaggedTemplateExpression> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
            Expression::ThisExpression(inner) => self
                .allocator
                .alloc(AstNode::<ThisExpression> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::UnaryExpression(inner) => self
                .allocator
                .alloc(AstNode::<UnaryExpression> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::UpdateExpression(inner) => self
                .allocator
                .alloc(AstNode::<UpdateExpression> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::YieldExpression(inner) => self
                .allocator
                .alloc(AstNode::<YieldExpression> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::PrivateInExpression(inner) => self
                .allocator
                .alloc(AstNode::<PrivateInExpression> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::JSXElement(inner) => self
                .allocator
                .alloc(AstNode::<JSXElement> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::JSXFragment(inner) => self
                .allocator
                .alloc(AstNode::<JSXFragment> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::TSAsExpression(inner) => self
                .allocator
                .alloc(AstNode::<TSAsExpression> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::TSSatisfiesExpression(inner) => self
                .allocator
                .alloc(AstNode::<TSSatisfiesExpression> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
            Expression::TSTypeAssertion(inner) => self
                .allocator
                .alloc(AstNode::<TSTypeAssertion> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::TSNonNullExpression(inner) => self
                .allocator
                .alloc(AstNode::<TSNonNullExpression> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Expression::TSInstantiationExpression(inner) => self
                .allocator
                .alloc(AstNode::<TSInstantiationExpression> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
            Expression::V8IntrinsicExpression(inner) => self
                .allocator
                .alloc(AstNode::<V8IntrinsicExpression> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
            it @ match_member_expression!(Expression) => self
                .allocator
                .alloc(AstNode::<'a, MemberExpression> {
                    inner: it.to_member_expression(),
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, Expression<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, IdentifierName<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn name(&self) -> Atom<'a> {
        self.inner.name
    }
}
impl<'a> AstNode<'a, IdentifierReference<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn name(&self) -> Atom<'a> {
        self.inner.name
    }
}
impl<'a> AstNode<'a, BindingIdentifier<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn name(&self) -> Atom<'a> {
        self.inner.name
    }
}
impl<'a> AstNode<'a, LabelIdentifier<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn name(&self) -> Atom<'a> {
        self.inner.name
    }
}
impl<'a> AstNode<'a, ThisExpression> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }
}
impl<'a> AstNode<'a, ArrayExpression<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn elements(&self) -> &AstNode<'a, Vec<'a, ArrayExpressionElement<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.elements,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ArrayExpression(transmute_self(self))),
        })
    }
}

impl<'a> AstNode<'a, ArrayExpressionElement<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.allocator.alloc(AstNodes::ArrayExpressionElement(transmute_self(self)));
        let node = match self.inner {
            ArrayExpressionElement::SpreadElement(s) => {
                AstNodes::SpreadElement(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            ArrayExpressionElement::Elision(s) => {
                AstNodes::Elision(self.allocator.alloc(AstNode {
                    inner: s,
                    parent,
                    allocator: self.allocator,
                }))
            }
            it @ match_expression!(ArrayExpressionElement) => {
                return self
                    .allocator
                    .alloc(AstNode { inner: it.to_expression(), parent, allocator: self.allocator })
                    .as_ast_nodes();
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, ArrayExpressionElement<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.allocator.alloc(AstNodes::ArrayExpressionElement(transmute_self(self)));
        match self.inner {
            ArrayExpressionElement::SpreadElement(inner) => self
                .allocator
                .alloc(AstNode::<SpreadElement> { inner, parent, allocator: self.allocator })
                .fmt(f),
            ArrayExpressionElement::Elision(inner) => self
                .allocator
                .alloc(AstNode::<Elision> { inner, parent, allocator: self.allocator })
                .fmt(f),
            it @ match_expression!(ArrayExpressionElement) => self
                .allocator
                .alloc(AstNode::<'a, Expression> {
                    inner: it.to_expression(),
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, ArrayExpressionElement<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, Elision> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }
}
impl<'a> AstNode<'a, ObjectExpression<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn properties(&self) -> &AstNode<'a, Vec<'a, ObjectPropertyKind<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.properties,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ObjectExpression(transmute_self(self))),
        })
    }
}

impl<'a> AstNode<'a, ObjectPropertyKind<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.parent;
        let node = match self.inner {
            ObjectPropertyKind::ObjectProperty(s) => {
                AstNodes::ObjectProperty(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            ObjectPropertyKind::SpreadProperty(s) => {
                AstNodes::SpreadElement(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, ObjectPropertyKind<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.parent;
        match self.inner {
            ObjectPropertyKind::ObjectProperty(inner) => self
                .allocator
                .alloc(AstNode::<ObjectProperty> { inner, parent, allocator: self.allocator })
                .fmt(f),
            ObjectPropertyKind::SpreadProperty(inner) => self
                .allocator
                .alloc(AstNode::<SpreadElement> { inner, parent, allocator: self.allocator })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, ObjectPropertyKind<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, ObjectProperty<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn kind(&self) -> PropertyKind {
        self.inner.kind
    }

    #[inline]
    pub fn key(&self) -> &AstNode<'a, PropertyKey<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.key,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ObjectProperty(transmute_self(self))),
        })
    }

    #[inline]
    pub fn value(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.value,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ObjectProperty(transmute_self(self))),
        })
    }

    #[inline]
    pub fn method(&self) -> bool {
        self.inner.method
    }

    #[inline]
    pub fn shorthand(&self) -> bool {
        self.inner.shorthand
    }

    #[inline]
    pub fn computed(&self) -> bool {
        self.inner.computed
    }
}

impl<'a> AstNode<'a, PropertyKey<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.allocator.alloc(AstNodes::PropertyKey(transmute_self(self)));
        let node = match self.inner {
            PropertyKey::StaticIdentifier(s) => {
                AstNodes::IdentifierName(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            PropertyKey::PrivateIdentifier(s) => {
                AstNodes::PrivateIdentifier(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            it @ match_expression!(PropertyKey) => {
                return self
                    .allocator
                    .alloc(AstNode { inner: it.to_expression(), parent, allocator: self.allocator })
                    .as_ast_nodes();
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, PropertyKey<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.allocator.alloc(AstNodes::PropertyKey(transmute_self(self)));
        match self.inner {
            PropertyKey::StaticIdentifier(inner) => self
                .allocator
                .alloc(AstNode::<IdentifierName> { inner, parent, allocator: self.allocator })
                .fmt(f),
            PropertyKey::PrivateIdentifier(inner) => self
                .allocator
                .alloc(AstNode::<PrivateIdentifier> { inner, parent, allocator: self.allocator })
                .fmt(f),
            it @ match_expression!(PropertyKey) => self
                .allocator
                .alloc(AstNode::<'a, Expression> {
                    inner: it.to_expression(),
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, PropertyKey<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, TemplateLiteral<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn quasis(&self) -> &AstNode<'a, Vec<'a, TemplateElement<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.quasis,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TemplateLiteral(transmute_self(self))),
        })
    }

    #[inline]
    pub fn expressions(&self) -> &AstNode<'a, Vec<'a, Expression<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.expressions,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TemplateLiteral(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, TaggedTemplateExpression<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn tag(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.tag,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TaggedTemplateExpression(transmute_self(self))),
        })
    }

    #[inline]
    pub fn type_arguments(&self) -> Option<&AstNode<'a, TSTypeParameterInstantiation<'a>>> {
        self.allocator
            .alloc(self.inner.type_arguments.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent:
                    self.allocator.alloc(AstNodes::TaggedTemplateExpression(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn quasi(&self) -> &AstNode<'a, TemplateLiteral<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.quasi,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TaggedTemplateExpression(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, TemplateElement<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn value(&self) -> &TemplateElementValue<'a> {
        &self.inner.value
    }

    #[inline]
    pub fn tail(&self) -> bool {
        self.inner.tail
    }

    #[inline]
    pub fn lone_surrogates(&self) -> bool {
        self.inner.lone_surrogates
    }
}

impl<'a> AstNode<'a, MemberExpression<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.allocator.alloc(AstNodes::MemberExpression(transmute_self(self)));
        let node = match self.inner {
            MemberExpression::ComputedMemberExpression(s) => {
                panic!(
                    "No Node for enum variant yet, please see `tasks/ast_tools/src/generators/ast_kind.rs`"
                )
            }
            MemberExpression::StaticMemberExpression(s) => {
                panic!(
                    "No Node for enum variant yet, please see `tasks/ast_tools/src/generators/ast_kind.rs`"
                )
            }
            MemberExpression::PrivateFieldExpression(s) => {
                panic!(
                    "No Node for enum variant yet, please see `tasks/ast_tools/src/generators/ast_kind.rs`"
                )
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, MemberExpression<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.allocator.alloc(AstNodes::MemberExpression(transmute_self(self)));
        match self.inner {
            MemberExpression::ComputedMemberExpression(inner) => self
                .allocator
                .alloc(AstNode::<ComputedMemberExpression> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
            MemberExpression::StaticMemberExpression(inner) => self
                .allocator
                .alloc(AstNode::<StaticMemberExpression> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
            MemberExpression::PrivateFieldExpression(inner) => self
                .allocator
                .alloc(AstNode::<PrivateFieldExpression> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, MemberExpression<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, ComputedMemberExpression<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn object(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.object,
            allocator: self.allocator,
            parent: self.parent,
        })
    }

    #[inline]
    pub fn expression(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.expression,
            allocator: self.allocator,
            parent: self.parent,
        })
    }

    #[inline]
    pub fn optional(&self) -> bool {
        self.inner.optional
    }
}
impl<'a> AstNode<'a, StaticMemberExpression<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn object(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.object,
            allocator: self.allocator,
            parent: self.parent,
        })
    }

    #[inline]
    pub fn property(&self) -> &AstNode<'a, IdentifierName<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.property,
            allocator: self.allocator,
            parent: self.parent,
        })
    }

    #[inline]
    pub fn optional(&self) -> bool {
        self.inner.optional
    }
}
impl<'a> AstNode<'a, PrivateFieldExpression<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn object(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.object,
            allocator: self.allocator,
            parent: self.parent,
        })
    }

    #[inline]
    pub fn field(&self) -> &AstNode<'a, PrivateIdentifier<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.field,
            allocator: self.allocator,
            parent: self.parent,
        })
    }

    #[inline]
    pub fn optional(&self) -> bool {
        self.inner.optional
    }
}
impl<'a> AstNode<'a, CallExpression<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn callee(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.callee,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::CallExpression(transmute_self(self))),
        })
    }

    #[inline]
    pub fn type_arguments(&self) -> Option<&AstNode<'a, TSTypeParameterInstantiation<'a>>> {
        self.allocator
            .alloc(self.inner.type_arguments.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::CallExpression(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn arguments(&self) -> &AstNode<'a, Vec<'a, Argument<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.arguments,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::CallExpression(transmute_self(self))),
        })
    }

    #[inline]
    pub fn optional(&self) -> bool {
        self.inner.optional
    }

    #[inline]
    pub fn pure(&self) -> bool {
        self.inner.pure
    }
}
impl<'a> AstNode<'a, NewExpression<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn callee(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.callee,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::NewExpression(transmute_self(self))),
        })
    }

    #[inline]
    pub fn type_arguments(&self) -> Option<&AstNode<'a, TSTypeParameterInstantiation<'a>>> {
        self.allocator
            .alloc(self.inner.type_arguments.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::NewExpression(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn arguments(&self) -> &AstNode<'a, Vec<'a, Argument<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.arguments,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::NewExpression(transmute_self(self))),
        })
    }

    #[inline]
    pub fn pure(&self) -> bool {
        self.inner.pure
    }
}
impl<'a> AstNode<'a, MetaProperty<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn meta(&self) -> &AstNode<'a, IdentifierName<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.meta,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::MetaProperty(transmute_self(self))),
        })
    }

    #[inline]
    pub fn property(&self) -> &AstNode<'a, IdentifierName<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.property,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::MetaProperty(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, SpreadElement<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn argument(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.argument,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::SpreadElement(transmute_self(self))),
        })
    }
}

impl<'a> AstNode<'a, Argument<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.allocator.alloc(AstNodes::Argument(transmute_self(self)));
        let node = match self.inner {
            Argument::SpreadElement(s) => AstNodes::SpreadElement(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            it @ match_expression!(Argument) => {
                return self
                    .allocator
                    .alloc(AstNode { inner: it.to_expression(), parent, allocator: self.allocator })
                    .as_ast_nodes();
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, Argument<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.allocator.alloc(AstNodes::Argument(transmute_self(self)));
        match self.inner {
            Argument::SpreadElement(inner) => self
                .allocator
                .alloc(AstNode::<SpreadElement> { inner, parent, allocator: self.allocator })
                .fmt(f),
            it @ match_expression!(Argument) => self
                .allocator
                .alloc(AstNode::<'a, Expression> {
                    inner: it.to_expression(),
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, Argument<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, UpdateExpression<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn operator(&self) -> UpdateOperator {
        self.inner.operator
    }

    #[inline]
    pub fn prefix(&self) -> bool {
        self.inner.prefix
    }

    #[inline]
    pub fn argument(&self) -> &AstNode<'a, SimpleAssignmentTarget<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.argument,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::UpdateExpression(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, UnaryExpression<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn operator(&self) -> UnaryOperator {
        self.inner.operator
    }

    #[inline]
    pub fn argument(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.argument,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::UnaryExpression(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, BinaryExpression<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn left(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.left,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::BinaryExpression(transmute_self(self))),
        })
    }

    #[inline]
    pub fn operator(&self) -> BinaryOperator {
        self.inner.operator
    }

    #[inline]
    pub fn right(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.right,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::BinaryExpression(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, PrivateInExpression<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn left(&self) -> &AstNode<'a, PrivateIdentifier<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.left,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::PrivateInExpression(transmute_self(self))),
        })
    }

    #[inline]
    pub fn right(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.right,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::PrivateInExpression(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, LogicalExpression<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn left(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.left,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::LogicalExpression(transmute_self(self))),
        })
    }

    #[inline]
    pub fn operator(&self) -> LogicalOperator {
        self.inner.operator
    }

    #[inline]
    pub fn right(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.right,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::LogicalExpression(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, ConditionalExpression<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn test(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.test,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ConditionalExpression(transmute_self(self))),
        })
    }

    #[inline]
    pub fn consequent(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.consequent,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ConditionalExpression(transmute_self(self))),
        })
    }

    #[inline]
    pub fn alternate(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.alternate,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ConditionalExpression(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, AssignmentExpression<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn operator(&self) -> AssignmentOperator {
        self.inner.operator
    }

    #[inline]
    pub fn left(&self) -> &AstNode<'a, AssignmentTarget<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.left,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::AssignmentExpression(transmute_self(self))),
        })
    }

    #[inline]
    pub fn right(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.right,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::AssignmentExpression(transmute_self(self))),
        })
    }
}

impl<'a> AstNode<'a, AssignmentTarget<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.allocator.alloc(AstNodes::AssignmentTarget(transmute_self(self)));
        let node = match self.inner {
            it @ match_simple_assignment_target!(AssignmentTarget) => {
                AstNodes::SimpleAssignmentTarget(self.allocator.alloc(AstNode {
                    inner: it.to_simple_assignment_target(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            it @ match_assignment_target_pattern!(AssignmentTarget) => {
                AstNodes::AssignmentTargetPattern(self.allocator.alloc(AstNode {
                    inner: it.to_assignment_target_pattern(),
                    parent,
                    allocator: self.allocator,
                }))
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, AssignmentTarget<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.allocator.alloc(AstNodes::AssignmentTarget(transmute_self(self)));
        match self.inner {
            it @ match_simple_assignment_target!(AssignmentTarget) => self
                .allocator
                .alloc(AstNode::<'a, SimpleAssignmentTarget> {
                    inner: it.to_simple_assignment_target(),
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
            it @ match_assignment_target_pattern!(AssignmentTarget) => self
                .allocator
                .alloc(AstNode::<'a, AssignmentTargetPattern> {
                    inner: it.to_assignment_target_pattern(),
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, AssignmentTarget<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}

impl<'a> AstNode<'a, SimpleAssignmentTarget<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.allocator.alloc(AstNodes::SimpleAssignmentTarget(transmute_self(self)));
        let node = match self.inner {
            SimpleAssignmentTarget::AssignmentTargetIdentifier(s) => {
                AstNodes::IdentifierReference(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            SimpleAssignmentTarget::TSAsExpression(s) => {
                AstNodes::TSAsExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            SimpleAssignmentTarget::TSSatisfiesExpression(s) => {
                AstNodes::TSSatisfiesExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            SimpleAssignmentTarget::TSNonNullExpression(s) => {
                AstNodes::TSNonNullExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            SimpleAssignmentTarget::TSTypeAssertion(s) => {
                AstNodes::TSTypeAssertion(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            it @ match_member_expression!(SimpleAssignmentTarget) => {
                AstNodes::MemberExpression(self.allocator.alloc(AstNode {
                    inner: it.to_member_expression(),
                    parent,
                    allocator: self.allocator,
                }))
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, SimpleAssignmentTarget<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.allocator.alloc(AstNodes::SimpleAssignmentTarget(transmute_self(self)));
        match self.inner {
            SimpleAssignmentTarget::AssignmentTargetIdentifier(inner) => self
                .allocator
                .alloc(AstNode::<IdentifierReference> { inner, parent, allocator: self.allocator })
                .fmt(f),
            SimpleAssignmentTarget::TSAsExpression(inner) => self
                .allocator
                .alloc(AstNode::<TSAsExpression> { inner, parent, allocator: self.allocator })
                .fmt(f),
            SimpleAssignmentTarget::TSSatisfiesExpression(inner) => self
                .allocator
                .alloc(AstNode::<TSSatisfiesExpression> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
            SimpleAssignmentTarget::TSNonNullExpression(inner) => self
                .allocator
                .alloc(AstNode::<TSNonNullExpression> { inner, parent, allocator: self.allocator })
                .fmt(f),
            SimpleAssignmentTarget::TSTypeAssertion(inner) => self
                .allocator
                .alloc(AstNode::<TSTypeAssertion> { inner, parent, allocator: self.allocator })
                .fmt(f),
            it @ match_member_expression!(SimpleAssignmentTarget) => self
                .allocator
                .alloc(AstNode::<'a, MemberExpression> {
                    inner: it.to_member_expression(),
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, SimpleAssignmentTarget<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}

impl<'a> AstNode<'a, AssignmentTargetPattern<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.allocator.alloc(AstNodes::AssignmentTargetPattern(transmute_self(self)));
        let node = match self.inner {
            AssignmentTargetPattern::ArrayAssignmentTarget(s) => {
                AstNodes::ArrayAssignmentTarget(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            AssignmentTargetPattern::ObjectAssignmentTarget(s) => {
                AstNodes::ObjectAssignmentTarget(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, AssignmentTargetPattern<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.allocator.alloc(AstNodes::AssignmentTargetPattern(transmute_self(self)));
        match self.inner {
            AssignmentTargetPattern::ArrayAssignmentTarget(inner) => self
                .allocator
                .alloc(AstNode::<ArrayAssignmentTarget> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
            AssignmentTargetPattern::ObjectAssignmentTarget(inner) => self
                .allocator
                .alloc(AstNode::<ObjectAssignmentTarget> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, AssignmentTargetPattern<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, ArrayAssignmentTarget<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn elements(&self) -> &AstNode<'a, Vec<'a, Option<AssignmentTargetMaybeDefault<'a>>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.elements,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ArrayAssignmentTarget(transmute_self(self))),
        })
    }

    #[inline]
    pub fn rest(&self) -> Option<&AstNode<'a, AssignmentTargetRest<'a>>> {
        self.allocator
            .alloc(self.inner.rest.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::ArrayAssignmentTarget(transmute_self(self))),
            }))
            .as_ref()
    }
}
impl<'a> AstNode<'a, ObjectAssignmentTarget<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn properties(&self) -> &AstNode<'a, Vec<'a, AssignmentTargetProperty<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.properties,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ObjectAssignmentTarget(transmute_self(self))),
        })
    }

    #[inline]
    pub fn rest(&self) -> Option<&AstNode<'a, AssignmentTargetRest<'a>>> {
        self.allocator
            .alloc(self.inner.rest.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent:
                    self.allocator.alloc(AstNodes::ObjectAssignmentTarget(transmute_self(self))),
            }))
            .as_ref()
    }
}
impl<'a> AstNode<'a, AssignmentTargetRest<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn target(&self) -> &AstNode<'a, AssignmentTarget<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.target,
            allocator: self.allocator,
            parent: self.parent,
        })
    }
}

impl<'a> AstNode<'a, AssignmentTargetMaybeDefault<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.parent;
        let node = match self.inner {
            AssignmentTargetMaybeDefault::AssignmentTargetWithDefault(s) => {
                AstNodes::AssignmentTargetWithDefault(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            it @ match_assignment_target!(AssignmentTargetMaybeDefault) => {
                AstNodes::AssignmentTarget(self.allocator.alloc(AstNode {
                    inner: it.to_assignment_target(),
                    parent,
                    allocator: self.allocator,
                }))
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, AssignmentTargetMaybeDefault<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.parent;
        match self.inner {
            AssignmentTargetMaybeDefault::AssignmentTargetWithDefault(inner) => self
                .allocator
                .alloc(AstNode::<AssignmentTargetWithDefault> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
            it @ match_assignment_target!(AssignmentTargetMaybeDefault) => self
                .allocator
                .alloc(AstNode::<'a, AssignmentTarget> {
                    inner: it.to_assignment_target(),
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, AssignmentTargetMaybeDefault<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, AssignmentTargetWithDefault<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn binding(&self) -> &AstNode<'a, AssignmentTarget<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.binding,
            allocator: self.allocator,
            parent: self
                .allocator
                .alloc(AstNodes::AssignmentTargetWithDefault(transmute_self(self))),
        })
    }

    #[inline]
    pub fn init(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.init,
            allocator: self.allocator,
            parent: self
                .allocator
                .alloc(AstNodes::AssignmentTargetWithDefault(transmute_self(self))),
        })
    }
}

impl<'a> AstNode<'a, AssignmentTargetProperty<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.parent;
        let node = match self.inner {
            AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(s) => {
                panic!(
                    "No Node for enum variant yet, please see `tasks/ast_tools/src/generators/ast_kind.rs`"
                )
            }
            AssignmentTargetProperty::AssignmentTargetPropertyProperty(s) => {
                panic!(
                    "No Node for enum variant yet, please see `tasks/ast_tools/src/generators/ast_kind.rs`"
                )
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, AssignmentTargetProperty<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.parent;
        match self.inner {
            AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(inner) => self
                .allocator
                .alloc(AstNode::<AssignmentTargetPropertyIdentifier> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
            AssignmentTargetProperty::AssignmentTargetPropertyProperty(inner) => self
                .allocator
                .alloc(AstNode::<AssignmentTargetPropertyProperty> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, AssignmentTargetProperty<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, AssignmentTargetPropertyIdentifier<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn binding(&self) -> &AstNode<'a, IdentifierReference<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.binding,
            allocator: self.allocator,
            parent: self.parent,
        })
    }

    #[inline]
    pub fn init(&self) -> Option<&AstNode<'a, Expression<'a>>> {
        self.allocator
            .alloc(self.inner.init.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.parent,
            }))
            .as_ref()
    }
}
impl<'a> AstNode<'a, AssignmentTargetPropertyProperty<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn name(&self) -> &AstNode<'a, PropertyKey<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.name,
            allocator: self.allocator,
            parent: self.parent,
        })
    }

    #[inline]
    pub fn binding(&self) -> &AstNode<'a, AssignmentTargetMaybeDefault<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.binding,
            allocator: self.allocator,
            parent: self.parent,
        })
    }

    #[inline]
    pub fn computed(&self) -> bool {
        self.inner.computed
    }
}
impl<'a> AstNode<'a, SequenceExpression<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn expressions(&self) -> &AstNode<'a, Vec<'a, Expression<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.expressions,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::SequenceExpression(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, Super> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }
}
impl<'a> AstNode<'a, AwaitExpression<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn argument(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.argument,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::AwaitExpression(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, ChainExpression<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn expression(&self) -> &AstNode<'a, ChainElement<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.expression,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ChainExpression(transmute_self(self))),
        })
    }
}

impl<'a> AstNode<'a, ChainElement<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.parent;
        let node = match self.inner {
            ChainElement::CallExpression(s) => {
                AstNodes::CallExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            ChainElement::TSNonNullExpression(s) => {
                AstNodes::TSNonNullExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            it @ match_member_expression!(ChainElement) => {
                AstNodes::MemberExpression(self.allocator.alloc(AstNode {
                    inner: it.to_member_expression(),
                    parent,
                    allocator: self.allocator,
                }))
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, ChainElement<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.parent;
        match self.inner {
            ChainElement::CallExpression(inner) => self
                .allocator
                .alloc(AstNode::<CallExpression> { inner, parent, allocator: self.allocator })
                .fmt(f),
            ChainElement::TSNonNullExpression(inner) => self
                .allocator
                .alloc(AstNode::<TSNonNullExpression> { inner, parent, allocator: self.allocator })
                .fmt(f),
            it @ match_member_expression!(ChainElement) => self
                .allocator
                .alloc(AstNode::<'a, MemberExpression> {
                    inner: it.to_member_expression(),
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, ChainElement<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, ParenthesizedExpression<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn expression(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.expression,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ParenthesizedExpression(transmute_self(self))),
        })
    }
}

impl<'a> AstNode<'a, Statement<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.parent;
        let node = match self.inner {
            Statement::BlockStatement(s) => {
                AstNodes::BlockStatement(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Statement::BreakStatement(s) => {
                AstNodes::BreakStatement(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Statement::ContinueStatement(s) => {
                AstNodes::ContinueStatement(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Statement::DebuggerStatement(s) => {
                AstNodes::DebuggerStatement(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Statement::DoWhileStatement(s) => {
                AstNodes::DoWhileStatement(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Statement::EmptyStatement(s) => {
                AstNodes::EmptyStatement(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Statement::ExpressionStatement(s) => {
                AstNodes::ExpressionStatement(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Statement::ForInStatement(s) => {
                AstNodes::ForInStatement(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Statement::ForOfStatement(s) => {
                AstNodes::ForOfStatement(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Statement::ForStatement(s) => AstNodes::ForStatement(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            Statement::IfStatement(s) => AstNodes::IfStatement(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            Statement::LabeledStatement(s) => {
                AstNodes::LabeledStatement(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Statement::ReturnStatement(s) => {
                AstNodes::ReturnStatement(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Statement::SwitchStatement(s) => {
                AstNodes::SwitchStatement(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Statement::ThrowStatement(s) => {
                AstNodes::ThrowStatement(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Statement::TryStatement(s) => AstNodes::TryStatement(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            Statement::WhileStatement(s) => {
                AstNodes::WhileStatement(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Statement::WithStatement(s) => AstNodes::WithStatement(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            it @ match_declaration!(Statement) => {
                return self
                    .allocator
                    .alloc(AstNode {
                        inner: it.to_declaration(),
                        parent,
                        allocator: self.allocator,
                    })
                    .as_ast_nodes();
            }
            it @ match_module_declaration!(Statement) => {
                AstNodes::ModuleDeclaration(self.allocator.alloc(AstNode {
                    inner: it.to_module_declaration(),
                    parent,
                    allocator: self.allocator,
                }))
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, Statement<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.parent;
        match self.inner {
            Statement::BlockStatement(inner) => self
                .allocator
                .alloc(AstNode::<BlockStatement> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Statement::BreakStatement(inner) => self
                .allocator
                .alloc(AstNode::<BreakStatement> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Statement::ContinueStatement(inner) => self
                .allocator
                .alloc(AstNode::<ContinueStatement> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Statement::DebuggerStatement(inner) => self
                .allocator
                .alloc(AstNode::<DebuggerStatement> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Statement::DoWhileStatement(inner) => self
                .allocator
                .alloc(AstNode::<DoWhileStatement> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Statement::EmptyStatement(inner) => self
                .allocator
                .alloc(AstNode::<EmptyStatement> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Statement::ExpressionStatement(inner) => self
                .allocator
                .alloc(AstNode::<ExpressionStatement> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Statement::ForInStatement(inner) => self
                .allocator
                .alloc(AstNode::<ForInStatement> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Statement::ForOfStatement(inner) => self
                .allocator
                .alloc(AstNode::<ForOfStatement> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Statement::ForStatement(inner) => self
                .allocator
                .alloc(AstNode::<ForStatement> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Statement::IfStatement(inner) => self
                .allocator
                .alloc(AstNode::<IfStatement> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Statement::LabeledStatement(inner) => self
                .allocator
                .alloc(AstNode::<LabeledStatement> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Statement::ReturnStatement(inner) => self
                .allocator
                .alloc(AstNode::<ReturnStatement> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Statement::SwitchStatement(inner) => self
                .allocator
                .alloc(AstNode::<SwitchStatement> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Statement::ThrowStatement(inner) => self
                .allocator
                .alloc(AstNode::<ThrowStatement> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Statement::TryStatement(inner) => self
                .allocator
                .alloc(AstNode::<TryStatement> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Statement::WhileStatement(inner) => self
                .allocator
                .alloc(AstNode::<WhileStatement> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Statement::WithStatement(inner) => self
                .allocator
                .alloc(AstNode::<WithStatement> { inner, parent, allocator: self.allocator })
                .fmt(f),
            it @ match_declaration!(Statement) => self
                .allocator
                .alloc(AstNode::<'a, Declaration> {
                    inner: it.to_declaration(),
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
            it @ match_module_declaration!(Statement) => self
                .allocator
                .alloc(AstNode::<'a, ModuleDeclaration> {
                    inner: it.to_module_declaration(),
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, Statement<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, Directive<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn expression(&self) -> &AstNode<'a, StringLiteral<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.expression,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::Directive(transmute_self(self))),
        })
    }

    #[inline]
    pub fn directive(&self) -> Atom<'a> {
        self.inner.directive
    }
}
impl<'a> AstNode<'a, Hashbang<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn value(&self) -> Atom<'a> {
        self.inner.value
    }
}
impl<'a> AstNode<'a, BlockStatement<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn body(&self) -> &AstNode<'a, Vec<'a, Statement<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.body,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::BlockStatement(transmute_self(self))),
        })
    }
}

impl<'a> AstNode<'a, Declaration<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.parent;
        let node = match self.inner {
            Declaration::VariableDeclaration(s) => {
                AstNodes::VariableDeclaration(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Declaration::FunctionDeclaration(s) => {
                AstNodes::Function(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Declaration::ClassDeclaration(s) => AstNodes::Class(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            Declaration::TSTypeAliasDeclaration(s) => {
                AstNodes::TSTypeAliasDeclaration(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Declaration::TSInterfaceDeclaration(s) => {
                AstNodes::TSInterfaceDeclaration(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Declaration::TSEnumDeclaration(s) => {
                AstNodes::TSEnumDeclaration(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Declaration::TSModuleDeclaration(s) => {
                AstNodes::TSModuleDeclaration(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            Declaration::TSImportEqualsDeclaration(s) => {
                AstNodes::TSImportEqualsDeclaration(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, Declaration<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.parent;
        match self.inner {
            Declaration::VariableDeclaration(inner) => self
                .allocator
                .alloc(AstNode::<VariableDeclaration> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Declaration::FunctionDeclaration(inner) => self
                .allocator
                .alloc(AstNode::<Function> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Declaration::ClassDeclaration(inner) => self
                .allocator
                .alloc(AstNode::<Class> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Declaration::TSTypeAliasDeclaration(inner) => self
                .allocator
                .alloc(AstNode::<TSTypeAliasDeclaration> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
            Declaration::TSInterfaceDeclaration(inner) => self
                .allocator
                .alloc(AstNode::<TSInterfaceDeclaration> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
            Declaration::TSEnumDeclaration(inner) => self
                .allocator
                .alloc(AstNode::<TSEnumDeclaration> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Declaration::TSModuleDeclaration(inner) => self
                .allocator
                .alloc(AstNode::<TSModuleDeclaration> { inner, parent, allocator: self.allocator })
                .fmt(f),
            Declaration::TSImportEqualsDeclaration(inner) => self
                .allocator
                .alloc(AstNode::<TSImportEqualsDeclaration> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, Declaration<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, VariableDeclaration<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn kind(&self) -> VariableDeclarationKind {
        self.inner.kind
    }

    #[inline]
    pub fn declarations(&self) -> &AstNode<'a, Vec<'a, VariableDeclarator<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.declarations,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::VariableDeclaration(transmute_self(self))),
        })
    }

    #[inline]
    pub fn declare(&self) -> bool {
        self.inner.declare
    }
}
impl<'a> AstNode<'a, VariableDeclarator<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn kind(&self) -> VariableDeclarationKind {
        self.inner.kind
    }

    #[inline]
    pub fn id(&self) -> &AstNode<'a, BindingPattern<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.id,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::VariableDeclarator(transmute_self(self))),
        })
    }

    #[inline]
    pub fn init(&self) -> Option<&AstNode<'a, Expression<'a>>> {
        self.allocator
            .alloc(self.inner.init.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::VariableDeclarator(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn definite(&self) -> bool {
        self.inner.definite
    }
}
impl<'a> AstNode<'a, EmptyStatement> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }
}
impl<'a> AstNode<'a, ExpressionStatement<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn expression(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.expression,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ExpressionStatement(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, IfStatement<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn test(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.test,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::IfStatement(transmute_self(self))),
        })
    }

    #[inline]
    pub fn consequent(&self) -> &AstNode<'a, Statement<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.consequent,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::IfStatement(transmute_self(self))),
        })
    }

    #[inline]
    pub fn alternate(&self) -> Option<&AstNode<'a, Statement<'a>>> {
        self.allocator
            .alloc(self.inner.alternate.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::IfStatement(transmute_self(self))),
            }))
            .as_ref()
    }
}
impl<'a> AstNode<'a, DoWhileStatement<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn body(&self) -> &AstNode<'a, Statement<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.body,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::DoWhileStatement(transmute_self(self))),
        })
    }

    #[inline]
    pub fn test(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.test,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::DoWhileStatement(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, WhileStatement<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn test(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.test,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::WhileStatement(transmute_self(self))),
        })
    }

    #[inline]
    pub fn body(&self) -> &AstNode<'a, Statement<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.body,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::WhileStatement(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, ForStatement<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn init(&self) -> Option<&AstNode<'a, ForStatementInit<'a>>> {
        self.allocator
            .alloc(self.inner.init.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::ForStatement(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn test(&self) -> Option<&AstNode<'a, Expression<'a>>> {
        self.allocator
            .alloc(self.inner.test.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::ForStatement(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn update(&self) -> Option<&AstNode<'a, Expression<'a>>> {
        self.allocator
            .alloc(self.inner.update.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::ForStatement(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn body(&self) -> &AstNode<'a, Statement<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.body,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ForStatement(transmute_self(self))),
        })
    }
}

impl<'a> AstNode<'a, ForStatementInit<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.allocator.alloc(AstNodes::ForStatementInit(transmute_self(self)));
        let node = match self.inner {
            ForStatementInit::VariableDeclaration(s) => {
                AstNodes::VariableDeclaration(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            it @ match_expression!(ForStatementInit) => {
                return self
                    .allocator
                    .alloc(AstNode { inner: it.to_expression(), parent, allocator: self.allocator })
                    .as_ast_nodes();
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, ForStatementInit<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.allocator.alloc(AstNodes::ForStatementInit(transmute_self(self)));
        match self.inner {
            ForStatementInit::VariableDeclaration(inner) => self
                .allocator
                .alloc(AstNode::<VariableDeclaration> { inner, parent, allocator: self.allocator })
                .fmt(f),
            it @ match_expression!(ForStatementInit) => self
                .allocator
                .alloc(AstNode::<'a, Expression> {
                    inner: it.to_expression(),
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, ForStatementInit<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, ForInStatement<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn left(&self) -> &AstNode<'a, ForStatementLeft<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.left,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ForInStatement(transmute_self(self))),
        })
    }

    #[inline]
    pub fn right(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.right,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ForInStatement(transmute_self(self))),
        })
    }

    #[inline]
    pub fn body(&self) -> &AstNode<'a, Statement<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.body,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ForInStatement(transmute_self(self))),
        })
    }
}

impl<'a> AstNode<'a, ForStatementLeft<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.parent;
        let node = match self.inner {
            ForStatementLeft::VariableDeclaration(s) => {
                AstNodes::VariableDeclaration(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            it @ match_assignment_target!(ForStatementLeft) => {
                AstNodes::AssignmentTarget(self.allocator.alloc(AstNode {
                    inner: it.to_assignment_target(),
                    parent,
                    allocator: self.allocator,
                }))
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, ForStatementLeft<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.parent;
        match self.inner {
            ForStatementLeft::VariableDeclaration(inner) => self
                .allocator
                .alloc(AstNode::<VariableDeclaration> { inner, parent, allocator: self.allocator })
                .fmt(f),
            it @ match_assignment_target!(ForStatementLeft) => self
                .allocator
                .alloc(AstNode::<'a, AssignmentTarget> {
                    inner: it.to_assignment_target(),
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, ForStatementLeft<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, ForOfStatement<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn r#await(&self) -> bool {
        self.inner.r#await
    }

    #[inline]
    pub fn left(&self) -> &AstNode<'a, ForStatementLeft<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.left,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ForOfStatement(transmute_self(self))),
        })
    }

    #[inline]
    pub fn right(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.right,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ForOfStatement(transmute_self(self))),
        })
    }

    #[inline]
    pub fn body(&self) -> &AstNode<'a, Statement<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.body,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ForOfStatement(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, ContinueStatement<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn label(&self) -> Option<&AstNode<'a, LabelIdentifier<'a>>> {
        self.allocator
            .alloc(self.inner.label.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::ContinueStatement(transmute_self(self))),
            }))
            .as_ref()
    }
}
impl<'a> AstNode<'a, BreakStatement<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn label(&self) -> Option<&AstNode<'a, LabelIdentifier<'a>>> {
        self.allocator
            .alloc(self.inner.label.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::BreakStatement(transmute_self(self))),
            }))
            .as_ref()
    }
}
impl<'a> AstNode<'a, ReturnStatement<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn argument(&self) -> Option<&AstNode<'a, Expression<'a>>> {
        self.allocator
            .alloc(self.inner.argument.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::ReturnStatement(transmute_self(self))),
            }))
            .as_ref()
    }
}
impl<'a> AstNode<'a, WithStatement<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn object(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.object,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::WithStatement(transmute_self(self))),
        })
    }

    #[inline]
    pub fn body(&self) -> &AstNode<'a, Statement<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.body,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::WithStatement(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, SwitchStatement<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn discriminant(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.discriminant,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::SwitchStatement(transmute_self(self))),
        })
    }

    #[inline]
    pub fn cases(&self) -> &AstNode<'a, Vec<'a, SwitchCase<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.cases,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::SwitchStatement(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, SwitchCase<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn test(&self) -> Option<&AstNode<'a, Expression<'a>>> {
        self.allocator
            .alloc(self.inner.test.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::SwitchCase(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn consequent(&self) -> &AstNode<'a, Vec<'a, Statement<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.consequent,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::SwitchCase(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, LabeledStatement<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn label(&self) -> &AstNode<'a, LabelIdentifier<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.label,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::LabeledStatement(transmute_self(self))),
        })
    }

    #[inline]
    pub fn body(&self) -> &AstNode<'a, Statement<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.body,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::LabeledStatement(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, ThrowStatement<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn argument(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.argument,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ThrowStatement(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, TryStatement<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn block(&self) -> &AstNode<'a, BlockStatement<'a>> {
        self.allocator.alloc(AstNode {
            inner: self.inner.block.as_ref(),
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TryStatement(transmute_self(self))),
        })
    }

    #[inline]
    pub fn handler(&self) -> Option<&AstNode<'a, CatchClause<'a>>> {
        self.allocator
            .alloc(self.inner.handler.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::TryStatement(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn finalizer(&self) -> Option<&AstNode<'a, BlockStatement<'a>>> {
        self.allocator
            .alloc(self.inner.finalizer.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::TryStatement(transmute_self(self))),
            }))
            .as_ref()
    }
}
impl<'a> AstNode<'a, CatchClause<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn param(&self) -> Option<&AstNode<'a, CatchParameter<'a>>> {
        self.allocator
            .alloc(self.inner.param.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::CatchClause(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn body(&self) -> &AstNode<'a, BlockStatement<'a>> {
        self.allocator.alloc(AstNode {
            inner: self.inner.body.as_ref(),
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::CatchClause(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, CatchParameter<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn pattern(&self) -> &AstNode<'a, BindingPattern<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.pattern,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::CatchParameter(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, DebuggerStatement> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }
}
impl<'a> AstNode<'a, BindingPattern<'a>> {
    #[inline]
    pub fn kind(&self) -> &AstNode<'a, BindingPatternKind<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.kind,
            allocator: self.allocator,
            parent: self.parent,
        })
    }

    #[inline]
    pub fn type_annotation(&self) -> Option<&AstNode<'a, TSTypeAnnotation<'a>>> {
        self.allocator
            .alloc(self.inner.type_annotation.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.parent,
            }))
            .as_ref()
    }

    #[inline]
    pub fn optional(&self) -> bool {
        self.inner.optional
    }
}

impl<'a> AstNode<'a, BindingPatternKind<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.parent;
        let node = match self.inner {
            BindingPatternKind::BindingIdentifier(s) => {
                AstNodes::BindingIdentifier(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            BindingPatternKind::ObjectPattern(s) => {
                AstNodes::ObjectPattern(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            BindingPatternKind::ArrayPattern(s) => {
                AstNodes::ArrayPattern(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            BindingPatternKind::AssignmentPattern(s) => {
                AstNodes::AssignmentPattern(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, BindingPatternKind<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.parent;
        match self.inner {
            BindingPatternKind::BindingIdentifier(inner) => self
                .allocator
                .alloc(AstNode::<BindingIdentifier> { inner, parent, allocator: self.allocator })
                .fmt(f),
            BindingPatternKind::ObjectPattern(inner) => self
                .allocator
                .alloc(AstNode::<ObjectPattern> { inner, parent, allocator: self.allocator })
                .fmt(f),
            BindingPatternKind::ArrayPattern(inner) => self
                .allocator
                .alloc(AstNode::<ArrayPattern> { inner, parent, allocator: self.allocator })
                .fmt(f),
            BindingPatternKind::AssignmentPattern(inner) => self
                .allocator
                .alloc(AstNode::<AssignmentPattern> { inner, parent, allocator: self.allocator })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, BindingPatternKind<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, AssignmentPattern<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn left(&self) -> &AstNode<'a, BindingPattern<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.left,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::AssignmentPattern(transmute_self(self))),
        })
    }

    #[inline]
    pub fn right(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.right,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::AssignmentPattern(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, ObjectPattern<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn properties(&self) -> &AstNode<'a, Vec<'a, BindingProperty<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.properties,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ObjectPattern(transmute_self(self))),
        })
    }

    #[inline]
    pub fn rest(&self) -> Option<&AstNode<'a, BindingRestElement<'a>>> {
        self.allocator
            .alloc(self.inner.rest.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::ObjectPattern(transmute_self(self))),
            }))
            .as_ref()
    }
}
impl<'a> AstNode<'a, BindingProperty<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn key(&self) -> &AstNode<'a, PropertyKey<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.key,
            allocator: self.allocator,
            parent: self.parent,
        })
    }

    #[inline]
    pub fn value(&self) -> &AstNode<'a, BindingPattern<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.value,
            allocator: self.allocator,
            parent: self.parent,
        })
    }

    #[inline]
    pub fn shorthand(&self) -> bool {
        self.inner.shorthand
    }

    #[inline]
    pub fn computed(&self) -> bool {
        self.inner.computed
    }
}
impl<'a> AstNode<'a, ArrayPattern<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn elements(&self) -> &AstNode<'a, Vec<'a, Option<BindingPattern<'a>>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.elements,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ArrayPattern(transmute_self(self))),
        })
    }

    #[inline]
    pub fn rest(&self) -> Option<&AstNode<'a, BindingRestElement<'a>>> {
        self.allocator
            .alloc(self.inner.rest.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::ArrayPattern(transmute_self(self))),
            }))
            .as_ref()
    }
}
impl<'a> AstNode<'a, BindingRestElement<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn argument(&self) -> &AstNode<'a, BindingPattern<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.argument,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::BindingRestElement(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, Function<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn r#type(&self) -> FunctionType {
        self.inner.r#type
    }

    #[inline]
    pub fn id(&self) -> Option<&AstNode<'a, BindingIdentifier<'a>>> {
        self.allocator
            .alloc(self.inner.id.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::Function(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn generator(&self) -> bool {
        self.inner.generator
    }

    #[inline]
    pub fn r#async(&self) -> bool {
        self.inner.r#async
    }

    #[inline]
    pub fn declare(&self) -> bool {
        self.inner.declare
    }

    #[inline]
    pub fn type_parameters(&self) -> Option<&AstNode<'a, TSTypeParameterDeclaration<'a>>> {
        self.allocator
            .alloc(self.inner.type_parameters.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::Function(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn this_param(&self) -> Option<&AstNode<'a, TSThisParameter<'a>>> {
        self.allocator
            .alloc(self.inner.this_param.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::Function(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn params(&self) -> &AstNode<'a, FormalParameters<'a>> {
        self.allocator.alloc(AstNode {
            inner: self.inner.params.as_ref(),
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::Function(transmute_self(self))),
        })
    }

    #[inline]
    pub fn return_type(&self) -> Option<&AstNode<'a, TSTypeAnnotation<'a>>> {
        self.allocator
            .alloc(self.inner.return_type.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::Function(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn body(&self) -> Option<&AstNode<'a, FunctionBody<'a>>> {
        self.allocator
            .alloc(self.inner.body.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::Function(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn pure(&self) -> bool {
        self.inner.pure
    }
}
impl<'a> AstNode<'a, FormalParameters<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn kind(&self) -> FormalParameterKind {
        self.inner.kind
    }

    #[inline]
    pub fn items(&self) -> &AstNode<'a, Vec<'a, FormalParameter<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.items,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::FormalParameters(transmute_self(self))),
        })
    }

    #[inline]
    pub fn rest(&self) -> Option<&AstNode<'a, BindingRestElement<'a>>> {
        self.allocator
            .alloc(self.inner.rest.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::FormalParameters(transmute_self(self))),
            }))
            .as_ref()
    }
}
impl<'a> AstNode<'a, FormalParameter<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn decorators(&self) -> &AstNode<'a, Vec<'a, Decorator<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.decorators,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::FormalParameter(transmute_self(self))),
        })
    }

    #[inline]
    pub fn pattern(&self) -> &AstNode<'a, BindingPattern<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.pattern,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::FormalParameter(transmute_self(self))),
        })
    }

    #[inline]
    pub fn accessibility(&self) -> Option<TSAccessibility> {
        self.inner.accessibility
    }

    #[inline]
    pub fn readonly(&self) -> bool {
        self.inner.readonly
    }

    #[inline]
    pub fn r#override(&self) -> bool {
        self.inner.r#override
    }
}
impl<'a> AstNode<'a, FunctionBody<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn directives(&self) -> &AstNode<'a, Vec<'a, Directive<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.directives,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::FunctionBody(transmute_self(self))),
        })
    }

    #[inline]
    pub fn statements(&self) -> &AstNode<'a, Vec<'a, Statement<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.statements,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::FunctionBody(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, ArrowFunctionExpression<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn expression(&self) -> bool {
        self.inner.expression
    }

    #[inline]
    pub fn r#async(&self) -> bool {
        self.inner.r#async
    }

    #[inline]
    pub fn type_parameters(&self) -> Option<&AstNode<'a, TSTypeParameterDeclaration<'a>>> {
        self.allocator
            .alloc(self.inner.type_parameters.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent:
                    self.allocator.alloc(AstNodes::ArrowFunctionExpression(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn params(&self) -> &AstNode<'a, FormalParameters<'a>> {
        self.allocator.alloc(AstNode {
            inner: self.inner.params.as_ref(),
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ArrowFunctionExpression(transmute_self(self))),
        })
    }

    #[inline]
    pub fn return_type(&self) -> Option<&AstNode<'a, TSTypeAnnotation<'a>>> {
        self.allocator
            .alloc(self.inner.return_type.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent:
                    self.allocator.alloc(AstNodes::ArrowFunctionExpression(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn body(&self) -> &AstNode<'a, FunctionBody<'a>> {
        self.allocator.alloc(AstNode {
            inner: self.inner.body.as_ref(),
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ArrowFunctionExpression(transmute_self(self))),
        })
    }

    #[inline]
    pub fn pure(&self) -> bool {
        self.inner.pure
    }
}
impl<'a> AstNode<'a, YieldExpression<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn delegate(&self) -> bool {
        self.inner.delegate
    }

    #[inline]
    pub fn argument(&self) -> Option<&AstNode<'a, Expression<'a>>> {
        self.allocator
            .alloc(self.inner.argument.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::YieldExpression(transmute_self(self))),
            }))
            .as_ref()
    }
}
impl<'a> AstNode<'a, Class<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn r#type(&self) -> ClassType {
        self.inner.r#type
    }

    #[inline]
    pub fn decorators(&self) -> &AstNode<'a, Vec<'a, Decorator<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.decorators,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::Class(transmute_self(self))),
        })
    }

    #[inline]
    pub fn id(&self) -> Option<&AstNode<'a, BindingIdentifier<'a>>> {
        self.allocator
            .alloc(self.inner.id.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::Class(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn type_parameters(&self) -> Option<&AstNode<'a, TSTypeParameterDeclaration<'a>>> {
        self.allocator
            .alloc(self.inner.type_parameters.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::Class(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn super_class(&self) -> Option<&AstNode<'a, Expression<'a>>> {
        self.allocator
            .alloc(self.inner.super_class.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::Class(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn super_type_arguments(&self) -> Option<&AstNode<'a, TSTypeParameterInstantiation<'a>>> {
        self.allocator
            .alloc(self.inner.super_type_arguments.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::Class(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn implements(&self) -> &AstNode<'a, Vec<'a, TSClassImplements<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.implements,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::Class(transmute_self(self))),
        })
    }

    #[inline]
    pub fn body(&self) -> &AstNode<'a, ClassBody<'a>> {
        self.allocator.alloc(AstNode {
            inner: self.inner.body.as_ref(),
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::Class(transmute_self(self))),
        })
    }

    #[inline]
    pub fn r#abstract(&self) -> bool {
        self.inner.r#abstract
    }

    #[inline]
    pub fn declare(&self) -> bool {
        self.inner.declare
    }
}
impl<'a> AstNode<'a, ClassBody<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn body(&self) -> &AstNode<'a, Vec<'a, ClassElement<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.body,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ClassBody(transmute_self(self))),
        })
    }
}

impl<'a> AstNode<'a, ClassElement<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.parent;
        let node = match self.inner {
            ClassElement::StaticBlock(s) => AstNodes::StaticBlock(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            ClassElement::MethodDefinition(s) => {
                AstNodes::MethodDefinition(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            ClassElement::PropertyDefinition(s) => {
                AstNodes::PropertyDefinition(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            ClassElement::AccessorProperty(s) => {
                panic!(
                    "No Node for enum variant yet, please see `tasks/ast_tools/src/generators/ast_kind.rs`"
                )
            }
            ClassElement::TSIndexSignature(s) => {
                panic!(
                    "No Node for enum variant yet, please see `tasks/ast_tools/src/generators/ast_kind.rs`"
                )
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, ClassElement<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.parent;
        match self.inner {
            ClassElement::StaticBlock(inner) => self
                .allocator
                .alloc(AstNode::<StaticBlock> { inner, parent, allocator: self.allocator })
                .fmt(f),
            ClassElement::MethodDefinition(inner) => self
                .allocator
                .alloc(AstNode::<MethodDefinition> { inner, parent, allocator: self.allocator })
                .fmt(f),
            ClassElement::PropertyDefinition(inner) => self
                .allocator
                .alloc(AstNode::<PropertyDefinition> { inner, parent, allocator: self.allocator })
                .fmt(f),
            ClassElement::AccessorProperty(inner) => self
                .allocator
                .alloc(AstNode::<AccessorProperty> { inner, parent, allocator: self.allocator })
                .fmt(f),
            ClassElement::TSIndexSignature(inner) => self
                .allocator
                .alloc(AstNode::<TSIndexSignature> { inner, parent, allocator: self.allocator })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, ClassElement<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, MethodDefinition<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn r#type(&self) -> MethodDefinitionType {
        self.inner.r#type
    }

    #[inline]
    pub fn decorators(&self) -> &AstNode<'a, Vec<'a, Decorator<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.decorators,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::MethodDefinition(transmute_self(self))),
        })
    }

    #[inline]
    pub fn key(&self) -> &AstNode<'a, PropertyKey<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.key,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::MethodDefinition(transmute_self(self))),
        })
    }

    #[inline]
    pub fn value(&self) -> &AstNode<'a, Function<'a>> {
        self.allocator.alloc(AstNode {
            inner: self.inner.value.as_ref(),
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::MethodDefinition(transmute_self(self))),
        })
    }

    #[inline]
    pub fn kind(&self) -> MethodDefinitionKind {
        self.inner.kind
    }

    #[inline]
    pub fn computed(&self) -> bool {
        self.inner.computed
    }

    #[inline]
    pub fn r#static(&self) -> bool {
        self.inner.r#static
    }

    #[inline]
    pub fn r#override(&self) -> bool {
        self.inner.r#override
    }

    #[inline]
    pub fn optional(&self) -> bool {
        self.inner.optional
    }

    #[inline]
    pub fn accessibility(&self) -> Option<TSAccessibility> {
        self.inner.accessibility
    }
}
impl<'a> AstNode<'a, PropertyDefinition<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn r#type(&self) -> PropertyDefinitionType {
        self.inner.r#type
    }

    #[inline]
    pub fn decorators(&self) -> &AstNode<'a, Vec<'a, Decorator<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.decorators,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::PropertyDefinition(transmute_self(self))),
        })
    }

    #[inline]
    pub fn key(&self) -> &AstNode<'a, PropertyKey<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.key,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::PropertyDefinition(transmute_self(self))),
        })
    }

    #[inline]
    pub fn type_annotation(&self) -> Option<&AstNode<'a, TSTypeAnnotation<'a>>> {
        self.allocator
            .alloc(self.inner.type_annotation.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::PropertyDefinition(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn value(&self) -> Option<&AstNode<'a, Expression<'a>>> {
        self.allocator
            .alloc(self.inner.value.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::PropertyDefinition(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn computed(&self) -> bool {
        self.inner.computed
    }

    #[inline]
    pub fn r#static(&self) -> bool {
        self.inner.r#static
    }

    #[inline]
    pub fn declare(&self) -> bool {
        self.inner.declare
    }

    #[inline]
    pub fn r#override(&self) -> bool {
        self.inner.r#override
    }

    #[inline]
    pub fn optional(&self) -> bool {
        self.inner.optional
    }

    #[inline]
    pub fn definite(&self) -> bool {
        self.inner.definite
    }

    #[inline]
    pub fn readonly(&self) -> bool {
        self.inner.readonly
    }

    #[inline]
    pub fn accessibility(&self) -> Option<TSAccessibility> {
        self.inner.accessibility
    }
}
impl<'a> AstNode<'a, PrivateIdentifier<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn name(&self) -> Atom<'a> {
        self.inner.name
    }
}
impl<'a> AstNode<'a, StaticBlock<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn body(&self) -> &AstNode<'a, Vec<'a, Statement<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.body,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::StaticBlock(transmute_self(self))),
        })
    }
}

impl<'a> AstNode<'a, ModuleDeclaration<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.allocator.alloc(AstNodes::ModuleDeclaration(transmute_self(self)));
        let node = match self.inner {
            ModuleDeclaration::ImportDeclaration(s) => {
                AstNodes::ImportDeclaration(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            ModuleDeclaration::ExportAllDeclaration(s) => {
                AstNodes::ExportAllDeclaration(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            ModuleDeclaration::ExportDefaultDeclaration(s) => {
                AstNodes::ExportDefaultDeclaration(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            ModuleDeclaration::ExportNamedDeclaration(s) => {
                AstNodes::ExportNamedDeclaration(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            ModuleDeclaration::TSExportAssignment(s) => {
                AstNodes::TSExportAssignment(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            ModuleDeclaration::TSNamespaceExportDeclaration(s) => {
                panic!(
                    "No Node for enum variant yet, please see `tasks/ast_tools/src/generators/ast_kind.rs`"
                )
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, ModuleDeclaration<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.allocator.alloc(AstNodes::ModuleDeclaration(transmute_self(self)));
        match self.inner {
            ModuleDeclaration::ImportDeclaration(inner) => self
                .allocator
                .alloc(AstNode::<ImportDeclaration> { inner, parent, allocator: self.allocator })
                .fmt(f),
            ModuleDeclaration::ExportAllDeclaration(inner) => self
                .allocator
                .alloc(AstNode::<ExportAllDeclaration> { inner, parent, allocator: self.allocator })
                .fmt(f),
            ModuleDeclaration::ExportDefaultDeclaration(inner) => self
                .allocator
                .alloc(AstNode::<ExportDefaultDeclaration> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
            ModuleDeclaration::ExportNamedDeclaration(inner) => self
                .allocator
                .alloc(AstNode::<ExportNamedDeclaration> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
            ModuleDeclaration::TSExportAssignment(inner) => self
                .allocator
                .alloc(AstNode::<TSExportAssignment> { inner, parent, allocator: self.allocator })
                .fmt(f),
            ModuleDeclaration::TSNamespaceExportDeclaration(inner) => self
                .allocator
                .alloc(AstNode::<TSNamespaceExportDeclaration> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, ModuleDeclaration<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, AccessorProperty<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn r#type(&self) -> AccessorPropertyType {
        self.inner.r#type
    }

    #[inline]
    pub fn decorators(&self) -> &AstNode<'a, Vec<'a, Decorator<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.decorators,
            allocator: self.allocator,
            parent: self.parent,
        })
    }

    #[inline]
    pub fn key(&self) -> &AstNode<'a, PropertyKey<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.key,
            allocator: self.allocator,
            parent: self.parent,
        })
    }

    #[inline]
    pub fn type_annotation(&self) -> Option<&AstNode<'a, TSTypeAnnotation<'a>>> {
        self.allocator
            .alloc(self.inner.type_annotation.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.parent,
            }))
            .as_ref()
    }

    #[inline]
    pub fn value(&self) -> Option<&AstNode<'a, Expression<'a>>> {
        self.allocator
            .alloc(self.inner.value.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.parent,
            }))
            .as_ref()
    }

    #[inline]
    pub fn computed(&self) -> bool {
        self.inner.computed
    }

    #[inline]
    pub fn r#static(&self) -> bool {
        self.inner.r#static
    }

    #[inline]
    pub fn r#override(&self) -> bool {
        self.inner.r#override
    }

    #[inline]
    pub fn definite(&self) -> bool {
        self.inner.definite
    }

    #[inline]
    pub fn accessibility(&self) -> Option<TSAccessibility> {
        self.inner.accessibility
    }
}
impl<'a> AstNode<'a, ImportExpression<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn source(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.source,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ImportExpression(transmute_self(self))),
        })
    }

    #[inline]
    pub fn options(&self) -> Option<&AstNode<'a, Expression<'a>>> {
        self.allocator
            .alloc(self.inner.options.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::ImportExpression(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn phase(&self) -> Option<ImportPhase> {
        self.inner.phase
    }
}
impl<'a> AstNode<'a, ImportDeclaration<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn specifiers(&self) -> Option<&AstNode<'a, Vec<'a, ImportDeclarationSpecifier<'a>>>> {
        self.allocator
            .alloc(self.inner.specifiers.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::ImportDeclaration(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn source(&self) -> &AstNode<'a, StringLiteral<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.source,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ImportDeclaration(transmute_self(self))),
        })
    }

    #[inline]
    pub fn phase(&self) -> Option<ImportPhase> {
        self.inner.phase
    }

    #[inline]
    pub fn with_clause(&self) -> Option<&AstNode<'a, WithClause<'a>>> {
        self.allocator
            .alloc(self.inner.with_clause.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::ImportDeclaration(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn import_kind(&self) -> ImportOrExportKind {
        self.inner.import_kind
    }
}

impl<'a> AstNode<'a, ImportDeclarationSpecifier<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.parent;
        let node = match self.inner {
            ImportDeclarationSpecifier::ImportSpecifier(s) => {
                AstNodes::ImportSpecifier(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            ImportDeclarationSpecifier::ImportDefaultSpecifier(s) => {
                AstNodes::ImportDefaultSpecifier(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            ImportDeclarationSpecifier::ImportNamespaceSpecifier(s) => {
                AstNodes::ImportNamespaceSpecifier(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, ImportDeclarationSpecifier<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.parent;
        match self.inner {
            ImportDeclarationSpecifier::ImportSpecifier(inner) => self
                .allocator
                .alloc(AstNode::<ImportSpecifier> { inner, parent, allocator: self.allocator })
                .fmt(f),
            ImportDeclarationSpecifier::ImportDefaultSpecifier(inner) => self
                .allocator
                .alloc(AstNode::<ImportDefaultSpecifier> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
            ImportDeclarationSpecifier::ImportNamespaceSpecifier(inner) => self
                .allocator
                .alloc(AstNode::<ImportNamespaceSpecifier> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, ImportDeclarationSpecifier<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, ImportSpecifier<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn imported(&self) -> &AstNode<'a, ModuleExportName<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.imported,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ImportSpecifier(transmute_self(self))),
        })
    }

    #[inline]
    pub fn local(&self) -> &AstNode<'a, BindingIdentifier<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.local,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ImportSpecifier(transmute_self(self))),
        })
    }

    #[inline]
    pub fn import_kind(&self) -> ImportOrExportKind {
        self.inner.import_kind
    }
}
impl<'a> AstNode<'a, ImportDefaultSpecifier<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn local(&self) -> &AstNode<'a, BindingIdentifier<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.local,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ImportDefaultSpecifier(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, ImportNamespaceSpecifier<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn local(&self) -> &AstNode<'a, BindingIdentifier<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.local,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ImportNamespaceSpecifier(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, WithClause<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn attributes_keyword(&self) -> &AstNode<'a, IdentifierName<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.attributes_keyword,
            allocator: self.allocator,
            parent: self.parent,
        })
    }

    #[inline]
    pub fn with_entries(&self) -> &AstNode<'a, Vec<'a, ImportAttribute<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.with_entries,
            allocator: self.allocator,
            parent: self.parent,
        })
    }
}
impl<'a> AstNode<'a, ImportAttribute<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn key(&self) -> &AstNode<'a, ImportAttributeKey<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.key,
            allocator: self.allocator,
            parent: self.parent,
        })
    }

    #[inline]
    pub fn value(&self) -> &AstNode<'a, StringLiteral<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.value,
            allocator: self.allocator,
            parent: self.parent,
        })
    }
}

impl<'a> AstNode<'a, ImportAttributeKey<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.parent;
        let node = match self.inner {
            ImportAttributeKey::Identifier(s) => {
                AstNodes::IdentifierName(self.allocator.alloc(AstNode {
                    inner: s,
                    parent,
                    allocator: self.allocator,
                }))
            }
            ImportAttributeKey::StringLiteral(s) => {
                AstNodes::StringLiteral(self.allocator.alloc(AstNode {
                    inner: s,
                    parent,
                    allocator: self.allocator,
                }))
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, ImportAttributeKey<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.parent;
        match self.inner {
            ImportAttributeKey::Identifier(inner) => self
                .allocator
                .alloc(AstNode::<IdentifierName> { inner, parent, allocator: self.allocator })
                .fmt(f),
            ImportAttributeKey::StringLiteral(inner) => self
                .allocator
                .alloc(AstNode::<StringLiteral> { inner, parent, allocator: self.allocator })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, ImportAttributeKey<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, ExportNamedDeclaration<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn declaration(&self) -> Option<&AstNode<'a, Declaration<'a>>> {
        self.allocator
            .alloc(self.inner.declaration.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent:
                    self.allocator.alloc(AstNodes::ExportNamedDeclaration(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn specifiers(&self) -> &AstNode<'a, Vec<'a, ExportSpecifier<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.specifiers,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ExportNamedDeclaration(transmute_self(self))),
        })
    }

    #[inline]
    pub fn source(&self) -> Option<&AstNode<'a, StringLiteral<'a>>> {
        self.allocator
            .alloc(self.inner.source.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent:
                    self.allocator.alloc(AstNodes::ExportNamedDeclaration(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn export_kind(&self) -> ImportOrExportKind {
        self.inner.export_kind
    }

    #[inline]
    pub fn with_clause(&self) -> Option<&AstNode<'a, WithClause<'a>>> {
        self.allocator
            .alloc(self.inner.with_clause.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent:
                    self.allocator.alloc(AstNodes::ExportNamedDeclaration(transmute_self(self))),
            }))
            .as_ref()
    }
}
impl<'a> AstNode<'a, ExportDefaultDeclaration<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn exported(&self) -> &AstNode<'a, ModuleExportName<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.exported,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ExportDefaultDeclaration(transmute_self(self))),
        })
    }

    #[inline]
    pub fn declaration(&self) -> &AstNode<'a, ExportDefaultDeclarationKind<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.declaration,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ExportDefaultDeclaration(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, ExportAllDeclaration<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn exported(&self) -> Option<&AstNode<'a, ModuleExportName<'a>>> {
        self.allocator
            .alloc(self.inner.exported.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::ExportAllDeclaration(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn source(&self) -> &AstNode<'a, StringLiteral<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.source,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ExportAllDeclaration(transmute_self(self))),
        })
    }

    #[inline]
    pub fn with_clause(&self) -> Option<&AstNode<'a, WithClause<'a>>> {
        self.allocator
            .alloc(self.inner.with_clause.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::ExportAllDeclaration(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn export_kind(&self) -> ImportOrExportKind {
        self.inner.export_kind
    }
}
impl<'a> AstNode<'a, ExportSpecifier<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn local(&self) -> &AstNode<'a, ModuleExportName<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.local,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ExportSpecifier(transmute_self(self))),
        })
    }

    #[inline]
    pub fn exported(&self) -> &AstNode<'a, ModuleExportName<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.exported,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::ExportSpecifier(transmute_self(self))),
        })
    }

    #[inline]
    pub fn export_kind(&self) -> ImportOrExportKind {
        self.inner.export_kind
    }
}

impl<'a> AstNode<'a, ExportDefaultDeclarationKind<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.parent;
        let node = match self.inner {
            ExportDefaultDeclarationKind::FunctionDeclaration(s) => {
                AstNodes::Function(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            ExportDefaultDeclarationKind::ClassDeclaration(s) => {
                AstNodes::Class(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            ExportDefaultDeclarationKind::TSInterfaceDeclaration(s) => {
                AstNodes::TSInterfaceDeclaration(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            it @ match_expression!(ExportDefaultDeclarationKind) => {
                return self
                    .allocator
                    .alloc(AstNode { inner: it.to_expression(), parent, allocator: self.allocator })
                    .as_ast_nodes();
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, ExportDefaultDeclarationKind<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.parent;
        match self.inner {
            ExportDefaultDeclarationKind::FunctionDeclaration(inner) => self
                .allocator
                .alloc(AstNode::<Function> { inner, parent, allocator: self.allocator })
                .fmt(f),
            ExportDefaultDeclarationKind::ClassDeclaration(inner) => self
                .allocator
                .alloc(AstNode::<Class> { inner, parent, allocator: self.allocator })
                .fmt(f),
            ExportDefaultDeclarationKind::TSInterfaceDeclaration(inner) => self
                .allocator
                .alloc(AstNode::<TSInterfaceDeclaration> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
            it @ match_expression!(ExportDefaultDeclarationKind) => self
                .allocator
                .alloc(AstNode::<'a, Expression> {
                    inner: it.to_expression(),
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, ExportDefaultDeclarationKind<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}

impl<'a> AstNode<'a, ModuleExportName<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.parent;
        let node = match self.inner {
            ModuleExportName::IdentifierName(s) => {
                AstNodes::IdentifierName(self.allocator.alloc(AstNode {
                    inner: s,
                    parent,
                    allocator: self.allocator,
                }))
            }
            ModuleExportName::IdentifierReference(s) => {
                AstNodes::IdentifierReference(self.allocator.alloc(AstNode {
                    inner: s,
                    parent,
                    allocator: self.allocator,
                }))
            }
            ModuleExportName::StringLiteral(s) => {
                AstNodes::StringLiteral(self.allocator.alloc(AstNode {
                    inner: s,
                    parent,
                    allocator: self.allocator,
                }))
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, ModuleExportName<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.parent;
        match self.inner {
            ModuleExportName::IdentifierName(inner) => self
                .allocator
                .alloc(AstNode::<IdentifierName> { inner, parent, allocator: self.allocator })
                .fmt(f),
            ModuleExportName::IdentifierReference(inner) => self
                .allocator
                .alloc(AstNode::<IdentifierReference> { inner, parent, allocator: self.allocator })
                .fmt(f),
            ModuleExportName::StringLiteral(inner) => self
                .allocator
                .alloc(AstNode::<StringLiteral> { inner, parent, allocator: self.allocator })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, ModuleExportName<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, V8IntrinsicExpression<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn name(&self) -> &AstNode<'a, IdentifierName<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.name,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::V8IntrinsicExpression(transmute_self(self))),
        })
    }

    #[inline]
    pub fn arguments(&self) -> &AstNode<'a, Vec<'a, Argument<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.arguments,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::V8IntrinsicExpression(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, BooleanLiteral> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn value(&self) -> bool {
        self.inner.value
    }
}
impl<'a> AstNode<'a, NullLiteral> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }
}
impl<'a> AstNode<'a, NumericLiteral<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn value(&self) -> f64 {
        self.inner.value
    }

    #[inline]
    pub fn raw(&self) -> Option<Atom<'a>> {
        self.inner.raw
    }

    #[inline]
    pub fn base(&self) -> NumberBase {
        self.inner.base
    }
}
impl<'a> AstNode<'a, StringLiteral<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn value(&self) -> Atom<'a> {
        self.inner.value
    }

    #[inline]
    pub fn raw(&self) -> Option<Atom<'a>> {
        self.inner.raw
    }

    #[inline]
    pub fn lone_surrogates(&self) -> bool {
        self.inner.lone_surrogates
    }
}
impl<'a> AstNode<'a, BigIntLiteral<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn raw(&self) -> Atom<'a> {
        self.inner.raw
    }

    #[inline]
    pub fn base(&self) -> BigintBase {
        self.inner.base
    }
}
impl<'a> AstNode<'a, RegExpLiteral<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn regex(&self) -> &RegExp<'a> {
        &self.inner.regex
    }

    #[inline]
    pub fn raw(&self) -> Option<Atom<'a>> {
        self.inner.raw
    }
}
impl<'a> AstNode<'a, JSXElement<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn opening_element(&self) -> &AstNode<'a, JSXOpeningElement<'a>> {
        self.allocator.alloc(AstNode {
            inner: self.inner.opening_element.as_ref(),
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::JSXElement(transmute_self(self))),
        })
    }

    #[inline]
    pub fn children(&self) -> &AstNode<'a, Vec<'a, JSXChild<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.children,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::JSXElement(transmute_self(self))),
        })
    }

    #[inline]
    pub fn closing_element(&self) -> Option<&AstNode<'a, JSXClosingElement<'a>>> {
        self.allocator
            .alloc(self.inner.closing_element.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::JSXElement(transmute_self(self))),
            }))
            .as_ref()
    }
}
impl<'a> AstNode<'a, JSXOpeningElement<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn name(&self) -> &AstNode<'a, JSXElementName<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.name,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::JSXOpeningElement(transmute_self(self))),
        })
    }

    #[inline]
    pub fn type_arguments(&self) -> Option<&AstNode<'a, TSTypeParameterInstantiation<'a>>> {
        self.allocator
            .alloc(self.inner.type_arguments.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::JSXOpeningElement(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn attributes(&self) -> &AstNode<'a, Vec<'a, JSXAttributeItem<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.attributes,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::JSXOpeningElement(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, JSXClosingElement<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn name(&self) -> &AstNode<'a, JSXElementName<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.name,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::JSXClosingElement(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, JSXFragment<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn opening_fragment(&self) -> &AstNode<'a, JSXOpeningFragment> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.opening_fragment,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::JSXFragment(transmute_self(self))),
        })
    }

    #[inline]
    pub fn children(&self) -> &AstNode<'a, Vec<'a, JSXChild<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.children,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::JSXFragment(transmute_self(self))),
        })
    }

    #[inline]
    pub fn closing_fragment(&self) -> &AstNode<'a, JSXClosingFragment> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.closing_fragment,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::JSXFragment(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, JSXOpeningFragment> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }
}
impl<'a> AstNode<'a, JSXClosingFragment> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }
}

impl<'a> AstNode<'a, JSXElementName<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.allocator.alloc(AstNodes::JSXElementName(transmute_self(self)));
        let node = match self.inner {
            JSXElementName::Identifier(s) => {
                AstNodes::JSXIdentifier(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            JSXElementName::IdentifierReference(s) => {
                AstNodes::IdentifierReference(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            JSXElementName::NamespacedName(s) => {
                AstNodes::JSXNamespacedName(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            JSXElementName::MemberExpression(s) => {
                AstNodes::JSXMemberExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            JSXElementName::ThisExpression(s) => {
                AstNodes::ThisExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, JSXElementName<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.allocator.alloc(AstNodes::JSXElementName(transmute_self(self)));
        match self.inner {
            JSXElementName::Identifier(inner) => self
                .allocator
                .alloc(AstNode::<JSXIdentifier> { inner, parent, allocator: self.allocator })
                .fmt(f),
            JSXElementName::IdentifierReference(inner) => self
                .allocator
                .alloc(AstNode::<IdentifierReference> { inner, parent, allocator: self.allocator })
                .fmt(f),
            JSXElementName::NamespacedName(inner) => self
                .allocator
                .alloc(AstNode::<JSXNamespacedName> { inner, parent, allocator: self.allocator })
                .fmt(f),
            JSXElementName::MemberExpression(inner) => self
                .allocator
                .alloc(AstNode::<JSXMemberExpression> { inner, parent, allocator: self.allocator })
                .fmt(f),
            JSXElementName::ThisExpression(inner) => self
                .allocator
                .alloc(AstNode::<ThisExpression> { inner, parent, allocator: self.allocator })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, JSXElementName<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, JSXNamespacedName<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn namespace(&self) -> &AstNode<'a, JSXIdentifier<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.namespace,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::JSXNamespacedName(transmute_self(self))),
        })
    }

    #[inline]
    pub fn name(&self) -> &AstNode<'a, JSXIdentifier<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.name,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::JSXNamespacedName(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, JSXMemberExpression<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn object(&self) -> &AstNode<'a, JSXMemberExpressionObject<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.object,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::JSXMemberExpression(transmute_self(self))),
        })
    }

    #[inline]
    pub fn property(&self) -> &AstNode<'a, JSXIdentifier<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.property,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::JSXMemberExpression(transmute_self(self))),
        })
    }
}

impl<'a> AstNode<'a, JSXMemberExpressionObject<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent =
            self.allocator.alloc(AstNodes::JSXMemberExpressionObject(transmute_self(self)));
        let node = match self.inner {
            JSXMemberExpressionObject::IdentifierReference(s) => {
                AstNodes::IdentifierReference(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            JSXMemberExpressionObject::MemberExpression(s) => {
                AstNodes::JSXMemberExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            JSXMemberExpressionObject::ThisExpression(s) => {
                AstNodes::ThisExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, JSXMemberExpressionObject<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent =
            self.allocator.alloc(AstNodes::JSXMemberExpressionObject(transmute_self(self)));
        match self.inner {
            JSXMemberExpressionObject::IdentifierReference(inner) => self
                .allocator
                .alloc(AstNode::<IdentifierReference> { inner, parent, allocator: self.allocator })
                .fmt(f),
            JSXMemberExpressionObject::MemberExpression(inner) => self
                .allocator
                .alloc(AstNode::<JSXMemberExpression> { inner, parent, allocator: self.allocator })
                .fmt(f),
            JSXMemberExpressionObject::ThisExpression(inner) => self
                .allocator
                .alloc(AstNode::<ThisExpression> { inner, parent, allocator: self.allocator })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, JSXMemberExpressionObject<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, JSXExpressionContainer<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn expression(&self) -> &AstNode<'a, JSXExpression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.expression,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::JSXExpressionContainer(transmute_self(self))),
        })
    }
}

impl<'a> AstNode<'a, JSXExpression<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.parent;
        let node = match self.inner {
            JSXExpression::EmptyExpression(s) => {
                panic!(
                    "No Node for enum variant yet, please see `tasks/ast_tools/src/generators/ast_kind.rs`"
                )
            }
            it @ match_expression!(JSXExpression) => {
                return self
                    .allocator
                    .alloc(AstNode { inner: it.to_expression(), parent, allocator: self.allocator })
                    .as_ast_nodes();
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, JSXExpression<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.parent;
        match self.inner {
            JSXExpression::EmptyExpression(inner) => self
                .allocator
                .alloc(AstNode::<JSXEmptyExpression> { inner, parent, allocator: self.allocator })
                .fmt(f),
            it @ match_expression!(JSXExpression) => self
                .allocator
                .alloc(AstNode::<'a, Expression> {
                    inner: it.to_expression(),
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, JSXExpression<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, JSXEmptyExpression> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }
}

impl<'a> AstNode<'a, JSXAttributeItem<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.allocator.alloc(AstNodes::JSXAttributeItem(transmute_self(self)));
        let node = match self.inner {
            JSXAttributeItem::Attribute(s) => {
                panic!(
                    "No Node for enum variant yet, please see `tasks/ast_tools/src/generators/ast_kind.rs`"
                )
            }
            JSXAttributeItem::SpreadAttribute(s) => {
                AstNodes::JSXSpreadAttribute(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, JSXAttributeItem<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.allocator.alloc(AstNodes::JSXAttributeItem(transmute_self(self)));
        match self.inner {
            JSXAttributeItem::Attribute(inner) => self
                .allocator
                .alloc(AstNode::<JSXAttribute> { inner, parent, allocator: self.allocator })
                .fmt(f),
            JSXAttributeItem::SpreadAttribute(inner) => self
                .allocator
                .alloc(AstNode::<JSXSpreadAttribute> { inner, parent, allocator: self.allocator })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, JSXAttributeItem<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, JSXAttribute<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn name(&self) -> &AstNode<'a, JSXAttributeName<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.name,
            allocator: self.allocator,
            parent: self.parent,
        })
    }

    #[inline]
    pub fn value(&self) -> Option<&AstNode<'a, JSXAttributeValue<'a>>> {
        self.allocator
            .alloc(self.inner.value.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.parent,
            }))
            .as_ref()
    }
}
impl<'a> AstNode<'a, JSXSpreadAttribute<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn argument(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.argument,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::JSXSpreadAttribute(transmute_self(self))),
        })
    }
}

impl<'a> AstNode<'a, JSXAttributeName<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.parent;
        let node = match self.inner {
            JSXAttributeName::Identifier(s) => {
                AstNodes::JSXIdentifier(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            JSXAttributeName::NamespacedName(s) => {
                AstNodes::JSXNamespacedName(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, JSXAttributeName<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.parent;
        match self.inner {
            JSXAttributeName::Identifier(inner) => self
                .allocator
                .alloc(AstNode::<JSXIdentifier> { inner, parent, allocator: self.allocator })
                .fmt(f),
            JSXAttributeName::NamespacedName(inner) => self
                .allocator
                .alloc(AstNode::<JSXNamespacedName> { inner, parent, allocator: self.allocator })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, JSXAttributeName<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}

impl<'a> AstNode<'a, JSXAttributeValue<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.parent;
        let node = match self.inner {
            JSXAttributeValue::StringLiteral(s) => {
                AstNodes::StringLiteral(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            JSXAttributeValue::ExpressionContainer(s) => {
                AstNodes::JSXExpressionContainer(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            JSXAttributeValue::Element(s) => AstNodes::JSXElement(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            JSXAttributeValue::Fragment(s) => {
                AstNodes::JSXFragment(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, JSXAttributeValue<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.parent;
        match self.inner {
            JSXAttributeValue::StringLiteral(inner) => self
                .allocator
                .alloc(AstNode::<StringLiteral> { inner, parent, allocator: self.allocator })
                .fmt(f),
            JSXAttributeValue::ExpressionContainer(inner) => self
                .allocator
                .alloc(AstNode::<JSXExpressionContainer> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
            JSXAttributeValue::Element(inner) => self
                .allocator
                .alloc(AstNode::<JSXElement> { inner, parent, allocator: self.allocator })
                .fmt(f),
            JSXAttributeValue::Fragment(inner) => self
                .allocator
                .alloc(AstNode::<JSXFragment> { inner, parent, allocator: self.allocator })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, JSXAttributeValue<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, JSXIdentifier<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn name(&self) -> Atom<'a> {
        self.inner.name
    }
}

impl<'a> AstNode<'a, JSXChild<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.parent;
        let node = match self.inner {
            JSXChild::Text(s) => AstNodes::JSXText(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            JSXChild::Element(s) => AstNodes::JSXElement(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            JSXChild::Fragment(s) => AstNodes::JSXFragment(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            JSXChild::ExpressionContainer(s) => {
                AstNodes::JSXExpressionContainer(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            JSXChild::Spread(s) => {
                panic!(
                    "No Node for enum variant yet, please see `tasks/ast_tools/src/generators/ast_kind.rs`"
                )
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, JSXChild<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.parent;
        match self.inner {
            JSXChild::Text(inner) => self
                .allocator
                .alloc(AstNode::<JSXText> { inner, parent, allocator: self.allocator })
                .fmt(f),
            JSXChild::Element(inner) => self
                .allocator
                .alloc(AstNode::<JSXElement> { inner, parent, allocator: self.allocator })
                .fmt(f),
            JSXChild::Fragment(inner) => self
                .allocator
                .alloc(AstNode::<JSXFragment> { inner, parent, allocator: self.allocator })
                .fmt(f),
            JSXChild::ExpressionContainer(inner) => self
                .allocator
                .alloc(AstNode::<JSXExpressionContainer> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
            JSXChild::Spread(inner) => self
                .allocator
                .alloc(AstNode::<JSXSpreadChild> { inner, parent, allocator: self.allocator })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, JSXChild<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, JSXSpreadChild<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn expression(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.expression,
            allocator: self.allocator,
            parent: self.parent,
        })
    }
}
impl<'a> AstNode<'a, JSXText<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn value(&self) -> Atom<'a> {
        self.inner.value
    }

    #[inline]
    pub fn raw(&self) -> Option<Atom<'a>> {
        self.inner.raw
    }
}
impl<'a> AstNode<'a, TSThisParameter<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn this_span(&self) -> Span {
        self.inner.this_span
    }

    #[inline]
    pub fn type_annotation(&self) -> Option<&AstNode<'a, TSTypeAnnotation<'a>>> {
        self.allocator
            .alloc(self.inner.type_annotation.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::TSThisParameter(transmute_self(self))),
            }))
            .as_ref()
    }
}
impl<'a> AstNode<'a, TSEnumDeclaration<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn id(&self) -> &AstNode<'a, BindingIdentifier<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.id,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSEnumDeclaration(transmute_self(self))),
        })
    }

    #[inline]
    pub fn body(&self) -> &AstNode<'a, TSEnumBody<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.body,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSEnumDeclaration(transmute_self(self))),
        })
    }

    #[inline]
    pub fn r#const(&self) -> bool {
        self.inner.r#const
    }

    #[inline]
    pub fn declare(&self) -> bool {
        self.inner.declare
    }
}
impl<'a> AstNode<'a, TSEnumBody<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn members(&self) -> &AstNode<'a, Vec<'a, TSEnumMember<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.members,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSEnumBody(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, TSEnumMember<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn id(&self) -> &AstNode<'a, TSEnumMemberName<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.id,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSEnumMember(transmute_self(self))),
        })
    }

    #[inline]
    pub fn initializer(&self) -> Option<&AstNode<'a, Expression<'a>>> {
        self.allocator
            .alloc(self.inner.initializer.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::TSEnumMember(transmute_self(self))),
            }))
            .as_ref()
    }
}

impl<'a> AstNode<'a, TSEnumMemberName<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.parent;
        let node = match self.inner {
            TSEnumMemberName::Identifier(s) => {
                AstNodes::IdentifierName(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            TSEnumMemberName::String(s) => AstNodes::StringLiteral(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            TSEnumMemberName::ComputedString(s) => {
                AstNodes::StringLiteral(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            TSEnumMemberName::ComputedTemplateString(s) => {
                AstNodes::TemplateLiteral(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, TSEnumMemberName<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.parent;
        match self.inner {
            TSEnumMemberName::Identifier(inner) => self
                .allocator
                .alloc(AstNode::<IdentifierName> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSEnumMemberName::String(inner) => self
                .allocator
                .alloc(AstNode::<StringLiteral> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSEnumMemberName::ComputedString(inner) => self
                .allocator
                .alloc(AstNode::<StringLiteral> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSEnumMemberName::ComputedTemplateString(inner) => self
                .allocator
                .alloc(AstNode::<TemplateLiteral> { inner, parent, allocator: self.allocator })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, TSEnumMemberName<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, TSTypeAnnotation<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn type_annotation(&self) -> &AstNode<'a, TSType<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.type_annotation,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSTypeAnnotation(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, TSLiteralType<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn literal(&self) -> &AstNode<'a, TSLiteral<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.literal,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSLiteralType(transmute_self(self))),
        })
    }
}

impl<'a> AstNode<'a, TSLiteral<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.parent;
        let node = match self.inner {
            TSLiteral::BooleanLiteral(s) => {
                AstNodes::BooleanLiteral(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            TSLiteral::NumericLiteral(s) => {
                AstNodes::NumericLiteral(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            TSLiteral::BigIntLiteral(s) => AstNodes::BigIntLiteral(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            TSLiteral::StringLiteral(s) => AstNodes::StringLiteral(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            TSLiteral::TemplateLiteral(s) => {
                AstNodes::TemplateLiteral(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            TSLiteral::UnaryExpression(s) => {
                AstNodes::UnaryExpression(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, TSLiteral<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.parent;
        match self.inner {
            TSLiteral::BooleanLiteral(inner) => self
                .allocator
                .alloc(AstNode::<BooleanLiteral> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSLiteral::NumericLiteral(inner) => self
                .allocator
                .alloc(AstNode::<NumericLiteral> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSLiteral::BigIntLiteral(inner) => self
                .allocator
                .alloc(AstNode::<BigIntLiteral> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSLiteral::StringLiteral(inner) => self
                .allocator
                .alloc(AstNode::<StringLiteral> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSLiteral::TemplateLiteral(inner) => self
                .allocator
                .alloc(AstNode::<TemplateLiteral> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSLiteral::UnaryExpression(inner) => self
                .allocator
                .alloc(AstNode::<UnaryExpression> { inner, parent, allocator: self.allocator })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, TSLiteral<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}

impl<'a> AstNode<'a, TSType<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.parent;
        let node = match self.inner {
            TSType::TSAnyKeyword(s) => AstNodes::TSAnyKeyword(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            TSType::TSBigIntKeyword(s) => {
                AstNodes::TSBigIntKeyword(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            TSType::TSBooleanKeyword(s) => {
                AstNodes::TSBooleanKeyword(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            TSType::TSIntrinsicKeyword(s) => {
                AstNodes::TSIntrinsicKeyword(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            TSType::TSNeverKeyword(s) => AstNodes::TSNeverKeyword(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            TSType::TSNullKeyword(s) => AstNodes::TSNullKeyword(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            TSType::TSNumberKeyword(s) => {
                AstNodes::TSNumberKeyword(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            TSType::TSObjectKeyword(s) => {
                AstNodes::TSObjectKeyword(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            TSType::TSStringKeyword(s) => {
                AstNodes::TSStringKeyword(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            TSType::TSSymbolKeyword(s) => {
                AstNodes::TSSymbolKeyword(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            TSType::TSUndefinedKeyword(s) => {
                AstNodes::TSUndefinedKeyword(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            TSType::TSUnknownKeyword(s) => {
                AstNodes::TSUnknownKeyword(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            TSType::TSVoidKeyword(s) => AstNodes::TSVoidKeyword(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            TSType::TSArrayType(s) => {
                panic!(
                    "No Node for enum variant yet, please see `tasks/ast_tools/src/generators/ast_kind.rs`"
                )
            }
            TSType::TSConditionalType(s) => {
                AstNodes::TSConditionalType(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            TSType::TSConstructorType(s) => {
                panic!(
                    "No Node for enum variant yet, please see `tasks/ast_tools/src/generators/ast_kind.rs`"
                )
            }
            TSType::TSFunctionType(s) => {
                panic!(
                    "No Node for enum variant yet, please see `tasks/ast_tools/src/generators/ast_kind.rs`"
                )
            }
            TSType::TSImportType(s) => AstNodes::TSImportType(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            TSType::TSIndexedAccessType(s) => {
                AstNodes::TSIndexedAccessType(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            TSType::TSInferType(s) => AstNodes::TSInferType(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            TSType::TSIntersectionType(s) => {
                AstNodes::TSIntersectionType(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            TSType::TSLiteralType(s) => AstNodes::TSLiteralType(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            TSType::TSMappedType(s) => AstNodes::TSMappedType(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            TSType::TSNamedTupleMember(s) => {
                AstNodes::TSNamedTupleMember(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            TSType::TSTemplateLiteralType(s) => {
                AstNodes::TSTemplateLiteralType(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            TSType::TSThisType(s) => AstNodes::TSThisType(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            TSType::TSTupleType(s) => {
                panic!(
                    "No Node for enum variant yet, please see `tasks/ast_tools/src/generators/ast_kind.rs`"
                )
            }
            TSType::TSTypeLiteral(s) => AstNodes::TSTypeLiteral(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            TSType::TSTypeOperatorType(s) => {
                panic!(
                    "No Node for enum variant yet, please see `tasks/ast_tools/src/generators/ast_kind.rs`"
                )
            }
            TSType::TSTypePredicate(s) => {
                panic!(
                    "No Node for enum variant yet, please see `tasks/ast_tools/src/generators/ast_kind.rs`"
                )
            }
            TSType::TSTypeQuery(s) => AstNodes::TSTypeQuery(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            TSType::TSTypeReference(s) => {
                AstNodes::TSTypeReference(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            TSType::TSUnionType(s) => AstNodes::TSUnionType(self.allocator.alloc(AstNode {
                inner: s.as_ref(),
                parent,
                allocator: self.allocator,
            })),
            TSType::TSParenthesizedType(s) => {
                AstNodes::TSParenthesizedType(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            TSType::JSDocNullableType(s) => {
                panic!(
                    "No Node for enum variant yet, please see `tasks/ast_tools/src/generators/ast_kind.rs`"
                )
            }
            TSType::JSDocNonNullableType(s) => {
                panic!(
                    "No Node for enum variant yet, please see `tasks/ast_tools/src/generators/ast_kind.rs`"
                )
            }
            TSType::JSDocUnknownType(s) => {
                panic!(
                    "No Node for enum variant yet, please see `tasks/ast_tools/src/generators/ast_kind.rs`"
                )
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, TSType<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.parent;
        match self.inner {
            TSType::TSAnyKeyword(inner) => self
                .allocator
                .alloc(AstNode::<TSAnyKeyword> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSBigIntKeyword(inner) => self
                .allocator
                .alloc(AstNode::<TSBigIntKeyword> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSBooleanKeyword(inner) => self
                .allocator
                .alloc(AstNode::<TSBooleanKeyword> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSIntrinsicKeyword(inner) => self
                .allocator
                .alloc(AstNode::<TSIntrinsicKeyword> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSNeverKeyword(inner) => self
                .allocator
                .alloc(AstNode::<TSNeverKeyword> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSNullKeyword(inner) => self
                .allocator
                .alloc(AstNode::<TSNullKeyword> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSNumberKeyword(inner) => self
                .allocator
                .alloc(AstNode::<TSNumberKeyword> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSObjectKeyword(inner) => self
                .allocator
                .alloc(AstNode::<TSObjectKeyword> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSStringKeyword(inner) => self
                .allocator
                .alloc(AstNode::<TSStringKeyword> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSSymbolKeyword(inner) => self
                .allocator
                .alloc(AstNode::<TSSymbolKeyword> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSUndefinedKeyword(inner) => self
                .allocator
                .alloc(AstNode::<TSUndefinedKeyword> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSUnknownKeyword(inner) => self
                .allocator
                .alloc(AstNode::<TSUnknownKeyword> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSVoidKeyword(inner) => self
                .allocator
                .alloc(AstNode::<TSVoidKeyword> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSArrayType(inner) => self
                .allocator
                .alloc(AstNode::<TSArrayType> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSConditionalType(inner) => self
                .allocator
                .alloc(AstNode::<TSConditionalType> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSConstructorType(inner) => self
                .allocator
                .alloc(AstNode::<TSConstructorType> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSFunctionType(inner) => self
                .allocator
                .alloc(AstNode::<TSFunctionType> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSImportType(inner) => self
                .allocator
                .alloc(AstNode::<TSImportType> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSIndexedAccessType(inner) => self
                .allocator
                .alloc(AstNode::<TSIndexedAccessType> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSInferType(inner) => self
                .allocator
                .alloc(AstNode::<TSInferType> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSIntersectionType(inner) => self
                .allocator
                .alloc(AstNode::<TSIntersectionType> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSLiteralType(inner) => self
                .allocator
                .alloc(AstNode::<TSLiteralType> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSMappedType(inner) => self
                .allocator
                .alloc(AstNode::<TSMappedType> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSNamedTupleMember(inner) => self
                .allocator
                .alloc(AstNode::<TSNamedTupleMember> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSTemplateLiteralType(inner) => self
                .allocator
                .alloc(AstNode::<TSTemplateLiteralType> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
            TSType::TSThisType(inner) => self
                .allocator
                .alloc(AstNode::<TSThisType> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSTupleType(inner) => self
                .allocator
                .alloc(AstNode::<TSTupleType> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSTypeLiteral(inner) => self
                .allocator
                .alloc(AstNode::<TSTypeLiteral> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSTypeOperatorType(inner) => self
                .allocator
                .alloc(AstNode::<TSTypeOperator> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSTypePredicate(inner) => self
                .allocator
                .alloc(AstNode::<TSTypePredicate> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSTypeQuery(inner) => self
                .allocator
                .alloc(AstNode::<TSTypeQuery> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSTypeReference(inner) => self
                .allocator
                .alloc(AstNode::<TSTypeReference> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSUnionType(inner) => self
                .allocator
                .alloc(AstNode::<TSUnionType> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::TSParenthesizedType(inner) => self
                .allocator
                .alloc(AstNode::<TSParenthesizedType> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::JSDocNullableType(inner) => self
                .allocator
                .alloc(AstNode::<JSDocNullableType> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::JSDocNonNullableType(inner) => self
                .allocator
                .alloc(AstNode::<JSDocNonNullableType> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSType::JSDocUnknownType(inner) => self
                .allocator
                .alloc(AstNode::<JSDocUnknownType> { inner, parent, allocator: self.allocator })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, TSType<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, TSConditionalType<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn check_type(&self) -> &AstNode<'a, TSType<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.check_type,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSConditionalType(transmute_self(self))),
        })
    }

    #[inline]
    pub fn extends_type(&self) -> &AstNode<'a, TSType<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.extends_type,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSConditionalType(transmute_self(self))),
        })
    }

    #[inline]
    pub fn true_type(&self) -> &AstNode<'a, TSType<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.true_type,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSConditionalType(transmute_self(self))),
        })
    }

    #[inline]
    pub fn false_type(&self) -> &AstNode<'a, TSType<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.false_type,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSConditionalType(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, TSUnionType<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn types(&self) -> &AstNode<'a, Vec<'a, TSType<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.types,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSUnionType(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, TSIntersectionType<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn types(&self) -> &AstNode<'a, Vec<'a, TSType<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.types,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSIntersectionType(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, TSParenthesizedType<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn type_annotation(&self) -> &AstNode<'a, TSType<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.type_annotation,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSParenthesizedType(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, TSTypeOperator<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn operator(&self) -> TSTypeOperatorOperator {
        self.inner.operator
    }

    #[inline]
    pub fn type_annotation(&self) -> &AstNode<'a, TSType<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.type_annotation,
            allocator: self.allocator,
            parent: self.parent,
        })
    }
}
impl<'a> AstNode<'a, TSArrayType<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn element_type(&self) -> &AstNode<'a, TSType<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.element_type,
            allocator: self.allocator,
            parent: self.parent,
        })
    }
}
impl<'a> AstNode<'a, TSIndexedAccessType<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn object_type(&self) -> &AstNode<'a, TSType<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.object_type,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSIndexedAccessType(transmute_self(self))),
        })
    }

    #[inline]
    pub fn index_type(&self) -> &AstNode<'a, TSType<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.index_type,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSIndexedAccessType(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, TSTupleType<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn element_types(&self) -> &AstNode<'a, Vec<'a, TSTupleElement<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.element_types,
            allocator: self.allocator,
            parent: self.parent,
        })
    }
}
impl<'a> AstNode<'a, TSNamedTupleMember<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn label(&self) -> &AstNode<'a, IdentifierName<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.label,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSNamedTupleMember(transmute_self(self))),
        })
    }

    #[inline]
    pub fn element_type(&self) -> &AstNode<'a, TSTupleElement<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.element_type,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSNamedTupleMember(transmute_self(self))),
        })
    }

    #[inline]
    pub fn optional(&self) -> bool {
        self.inner.optional
    }
}
impl<'a> AstNode<'a, TSOptionalType<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn type_annotation(&self) -> &AstNode<'a, TSType<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.type_annotation,
            allocator: self.allocator,
            parent: self.parent,
        })
    }
}
impl<'a> AstNode<'a, TSRestType<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn type_annotation(&self) -> &AstNode<'a, TSType<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.type_annotation,
            allocator: self.allocator,
            parent: self.parent,
        })
    }
}

impl<'a> AstNode<'a, TSTupleElement<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.parent;
        let node = match self.inner {
            TSTupleElement::TSOptionalType(s) => {
                panic!(
                    "No Node for enum variant yet, please see `tasks/ast_tools/src/generators/ast_kind.rs`"
                )
            }
            TSTupleElement::TSRestType(s) => {
                panic!(
                    "No Node for enum variant yet, please see `tasks/ast_tools/src/generators/ast_kind.rs`"
                )
            }
            it @ match_ts_type!(TSTupleElement) => {
                return self
                    .allocator
                    .alloc(AstNode { inner: it.to_ts_type(), parent, allocator: self.allocator })
                    .as_ast_nodes();
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, TSTupleElement<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.parent;
        match self.inner {
            TSTupleElement::TSOptionalType(inner) => self
                .allocator
                .alloc(AstNode::<TSOptionalType> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSTupleElement::TSRestType(inner) => self
                .allocator
                .alloc(AstNode::<TSRestType> { inner, parent, allocator: self.allocator })
                .fmt(f),
            it @ match_ts_type!(TSTupleElement) => self
                .allocator
                .alloc(AstNode::<'a, TSType> {
                    inner: it.to_ts_type(),
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, TSTupleElement<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, TSAnyKeyword> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }
}
impl<'a> AstNode<'a, TSStringKeyword> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }
}
impl<'a> AstNode<'a, TSBooleanKeyword> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }
}
impl<'a> AstNode<'a, TSNumberKeyword> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }
}
impl<'a> AstNode<'a, TSNeverKeyword> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }
}
impl<'a> AstNode<'a, TSIntrinsicKeyword> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }
}
impl<'a> AstNode<'a, TSUnknownKeyword> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }
}
impl<'a> AstNode<'a, TSNullKeyword> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }
}
impl<'a> AstNode<'a, TSUndefinedKeyword> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }
}
impl<'a> AstNode<'a, TSVoidKeyword> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }
}
impl<'a> AstNode<'a, TSSymbolKeyword> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }
}
impl<'a> AstNode<'a, TSThisType> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }
}
impl<'a> AstNode<'a, TSObjectKeyword> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }
}
impl<'a> AstNode<'a, TSBigIntKeyword> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }
}
impl<'a> AstNode<'a, TSTypeReference<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn type_name(&self) -> &AstNode<'a, TSTypeName<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.type_name,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSTypeReference(transmute_self(self))),
        })
    }

    #[inline]
    pub fn type_arguments(&self) -> Option<&AstNode<'a, TSTypeParameterInstantiation<'a>>> {
        self.allocator
            .alloc(self.inner.type_arguments.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::TSTypeReference(transmute_self(self))),
            }))
            .as_ref()
    }
}

impl<'a> AstNode<'a, TSTypeName<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.allocator.alloc(AstNodes::TSTypeName(transmute_self(self)));
        let node = match self.inner {
            TSTypeName::IdentifierReference(s) => {
                AstNodes::IdentifierReference(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            TSTypeName::QualifiedName(s) => {
                AstNodes::TSQualifiedName(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, TSTypeName<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.allocator.alloc(AstNodes::TSTypeName(transmute_self(self)));
        match self.inner {
            TSTypeName::IdentifierReference(inner) => self
                .allocator
                .alloc(AstNode::<IdentifierReference> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSTypeName::QualifiedName(inner) => self
                .allocator
                .alloc(AstNode::<TSQualifiedName> { inner, parent, allocator: self.allocator })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, TSTypeName<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, TSQualifiedName<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn left(&self) -> &AstNode<'a, TSTypeName<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.left,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSQualifiedName(transmute_self(self))),
        })
    }

    #[inline]
    pub fn right(&self) -> &AstNode<'a, IdentifierName<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.right,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSQualifiedName(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, TSTypeParameterInstantiation<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn params(&self) -> &AstNode<'a, Vec<'a, TSType<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.params,
            allocator: self.allocator,
            parent: self
                .allocator
                .alloc(AstNodes::TSTypeParameterInstantiation(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, TSTypeParameter<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn name(&self) -> &AstNode<'a, BindingIdentifier<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.name,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSTypeParameter(transmute_self(self))),
        })
    }

    #[inline]
    pub fn constraint(&self) -> Option<&AstNode<'a, TSType<'a>>> {
        self.allocator
            .alloc(self.inner.constraint.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::TSTypeParameter(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn default(&self) -> Option<&AstNode<'a, TSType<'a>>> {
        self.allocator
            .alloc(self.inner.default.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::TSTypeParameter(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn r#in(&self) -> bool {
        self.inner.r#in
    }

    #[inline]
    pub fn out(&self) -> bool {
        self.inner.out
    }

    #[inline]
    pub fn r#const(&self) -> bool {
        self.inner.r#const
    }
}
impl<'a> AstNode<'a, TSTypeParameterDeclaration<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn params(&self) -> &AstNode<'a, Vec<'a, TSTypeParameter<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.params,
            allocator: self.allocator,
            parent: self
                .allocator
                .alloc(AstNodes::TSTypeParameterDeclaration(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, TSTypeAliasDeclaration<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn id(&self) -> &AstNode<'a, BindingIdentifier<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.id,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSTypeAliasDeclaration(transmute_self(self))),
        })
    }

    #[inline]
    pub fn type_parameters(&self) -> Option<&AstNode<'a, TSTypeParameterDeclaration<'a>>> {
        self.allocator
            .alloc(self.inner.type_parameters.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent:
                    self.allocator.alloc(AstNodes::TSTypeAliasDeclaration(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn type_annotation(&self) -> &AstNode<'a, TSType<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.type_annotation,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSTypeAliasDeclaration(transmute_self(self))),
        })
    }

    #[inline]
    pub fn declare(&self) -> bool {
        self.inner.declare
    }
}
impl<'a> AstNode<'a, TSClassImplements<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn expression(&self) -> &AstNode<'a, TSTypeName<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.expression,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSClassImplements(transmute_self(self))),
        })
    }

    #[inline]
    pub fn type_arguments(&self) -> Option<&AstNode<'a, TSTypeParameterInstantiation<'a>>> {
        self.allocator
            .alloc(self.inner.type_arguments.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::TSClassImplements(transmute_self(self))),
            }))
            .as_ref()
    }
}
impl<'a> AstNode<'a, TSInterfaceDeclaration<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn id(&self) -> &AstNode<'a, BindingIdentifier<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.id,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSInterfaceDeclaration(transmute_self(self))),
        })
    }

    #[inline]
    pub fn type_parameters(&self) -> Option<&AstNode<'a, TSTypeParameterDeclaration<'a>>> {
        self.allocator
            .alloc(self.inner.type_parameters.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent:
                    self.allocator.alloc(AstNodes::TSInterfaceDeclaration(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn extends(&self) -> &AstNode<'a, Vec<'a, TSInterfaceHeritage<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.extends,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSInterfaceDeclaration(transmute_self(self))),
        })
    }

    #[inline]
    pub fn body(&self) -> &AstNode<'a, TSInterfaceBody<'a>> {
        self.allocator.alloc(AstNode {
            inner: self.inner.body.as_ref(),
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSInterfaceDeclaration(transmute_self(self))),
        })
    }

    #[inline]
    pub fn declare(&self) -> bool {
        self.inner.declare
    }
}
impl<'a> AstNode<'a, TSInterfaceBody<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn body(&self) -> &AstNode<'a, Vec<'a, TSSignature<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.body,
            allocator: self.allocator,
            parent: self.parent,
        })
    }
}
impl<'a> AstNode<'a, TSPropertySignature<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn computed(&self) -> bool {
        self.inner.computed
    }

    #[inline]
    pub fn optional(&self) -> bool {
        self.inner.optional
    }

    #[inline]
    pub fn readonly(&self) -> bool {
        self.inner.readonly
    }

    #[inline]
    pub fn key(&self) -> &AstNode<'a, PropertyKey<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.key,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSPropertySignature(transmute_self(self))),
        })
    }

    #[inline]
    pub fn type_annotation(&self) -> Option<&AstNode<'a, TSTypeAnnotation<'a>>> {
        self.allocator
            .alloc(self.inner.type_annotation.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::TSPropertySignature(transmute_self(self))),
            }))
            .as_ref()
    }
}

impl<'a> AstNode<'a, TSSignature<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.parent;
        let node = match self.inner {
            TSSignature::TSIndexSignature(s) => {
                panic!(
                    "No Node for enum variant yet, please see `tasks/ast_tools/src/generators/ast_kind.rs`"
                )
            }
            TSSignature::TSPropertySignature(s) => {
                AstNodes::TSPropertySignature(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            TSSignature::TSCallSignatureDeclaration(s) => {
                panic!(
                    "No Node for enum variant yet, please see `tasks/ast_tools/src/generators/ast_kind.rs`"
                )
            }
            TSSignature::TSConstructSignatureDeclaration(s) => {
                AstNodes::TSConstructSignatureDeclaration(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            TSSignature::TSMethodSignature(s) => {
                AstNodes::TSMethodSignature(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, TSSignature<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.parent;
        match self.inner {
            TSSignature::TSIndexSignature(inner) => self
                .allocator
                .alloc(AstNode::<TSIndexSignature> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSSignature::TSPropertySignature(inner) => self
                .allocator
                .alloc(AstNode::<TSPropertySignature> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSSignature::TSCallSignatureDeclaration(inner) => self
                .allocator
                .alloc(AstNode::<TSCallSignatureDeclaration> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
            TSSignature::TSConstructSignatureDeclaration(inner) => self
                .allocator
                .alloc(AstNode::<TSConstructSignatureDeclaration> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
            TSSignature::TSMethodSignature(inner) => self
                .allocator
                .alloc(AstNode::<TSMethodSignature> { inner, parent, allocator: self.allocator })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, TSSignature<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, TSIndexSignature<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn parameters(&self) -> &AstNode<'a, Vec<'a, TSIndexSignatureName<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.parameters,
            allocator: self.allocator,
            parent: self.parent,
        })
    }

    #[inline]
    pub fn type_annotation(&self) -> &AstNode<'a, TSTypeAnnotation<'a>> {
        self.allocator.alloc(AstNode {
            inner: self.inner.type_annotation.as_ref(),
            allocator: self.allocator,
            parent: self.parent,
        })
    }

    #[inline]
    pub fn readonly(&self) -> bool {
        self.inner.readonly
    }

    #[inline]
    pub fn r#static(&self) -> bool {
        self.inner.r#static
    }
}
impl<'a> AstNode<'a, TSCallSignatureDeclaration<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn type_parameters(&self) -> Option<&AstNode<'a, TSTypeParameterDeclaration<'a>>> {
        self.allocator
            .alloc(self.inner.type_parameters.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.parent,
            }))
            .as_ref()
    }

    #[inline]
    pub fn this_param(&self) -> Option<&AstNode<'a, TSThisParameter<'a>>> {
        self.allocator
            .alloc(self.inner.this_param.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.parent,
            }))
            .as_ref()
    }

    #[inline]
    pub fn params(&self) -> &AstNode<'a, FormalParameters<'a>> {
        self.allocator.alloc(AstNode {
            inner: self.inner.params.as_ref(),
            allocator: self.allocator,
            parent: self.parent,
        })
    }

    #[inline]
    pub fn return_type(&self) -> Option<&AstNode<'a, TSTypeAnnotation<'a>>> {
        self.allocator
            .alloc(self.inner.return_type.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.parent,
            }))
            .as_ref()
    }
}
impl<'a> AstNode<'a, TSMethodSignature<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn key(&self) -> &AstNode<'a, PropertyKey<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.key,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSMethodSignature(transmute_self(self))),
        })
    }

    #[inline]
    pub fn computed(&self) -> bool {
        self.inner.computed
    }

    #[inline]
    pub fn optional(&self) -> bool {
        self.inner.optional
    }

    #[inline]
    pub fn kind(&self) -> TSMethodSignatureKind {
        self.inner.kind
    }

    #[inline]
    pub fn type_parameters(&self) -> Option<&AstNode<'a, TSTypeParameterDeclaration<'a>>> {
        self.allocator
            .alloc(self.inner.type_parameters.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::TSMethodSignature(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn this_param(&self) -> Option<&AstNode<'a, TSThisParameter<'a>>> {
        self.allocator
            .alloc(self.inner.this_param.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::TSMethodSignature(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn params(&self) -> &AstNode<'a, FormalParameters<'a>> {
        self.allocator.alloc(AstNode {
            inner: self.inner.params.as_ref(),
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSMethodSignature(transmute_self(self))),
        })
    }

    #[inline]
    pub fn return_type(&self) -> Option<&AstNode<'a, TSTypeAnnotation<'a>>> {
        self.allocator
            .alloc(self.inner.return_type.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::TSMethodSignature(transmute_self(self))),
            }))
            .as_ref()
    }
}
impl<'a> AstNode<'a, TSConstructSignatureDeclaration<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn type_parameters(&self) -> Option<&AstNode<'a, TSTypeParameterDeclaration<'a>>> {
        self.allocator
            .alloc(self.inner.type_parameters.as_ref().map(|inner| {
                AstNode {
                    inner: inner.as_ref(),
                    allocator: self.allocator,
                    parent: self
                        .allocator
                        .alloc(AstNodes::TSConstructSignatureDeclaration(transmute_self(self))),
                }
            }))
            .as_ref()
    }

    #[inline]
    pub fn params(&self) -> &AstNode<'a, FormalParameters<'a>> {
        self.allocator.alloc(AstNode {
            inner: self.inner.params.as_ref(),
            allocator: self.allocator,
            parent: self
                .allocator
                .alloc(AstNodes::TSConstructSignatureDeclaration(transmute_self(self))),
        })
    }

    #[inline]
    pub fn return_type(&self) -> Option<&AstNode<'a, TSTypeAnnotation<'a>>> {
        self.allocator
            .alloc(self.inner.return_type.as_ref().map(|inner| {
                AstNode {
                    inner: inner.as_ref(),
                    allocator: self.allocator,
                    parent: self
                        .allocator
                        .alloc(AstNodes::TSConstructSignatureDeclaration(transmute_self(self))),
                }
            }))
            .as_ref()
    }
}
impl<'a> AstNode<'a, TSIndexSignatureName<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn name(&self) -> Atom<'a> {
        self.inner.name
    }

    #[inline]
    pub fn type_annotation(&self) -> &AstNode<'a, TSTypeAnnotation<'a>> {
        self.allocator.alloc(AstNode {
            inner: self.inner.type_annotation.as_ref(),
            allocator: self.allocator,
            parent: self.parent,
        })
    }
}
impl<'a> AstNode<'a, TSInterfaceHeritage<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn expression(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.expression,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSInterfaceHeritage(transmute_self(self))),
        })
    }

    #[inline]
    pub fn type_arguments(&self) -> Option<&AstNode<'a, TSTypeParameterInstantiation<'a>>> {
        self.allocator
            .alloc(self.inner.type_arguments.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::TSInterfaceHeritage(transmute_self(self))),
            }))
            .as_ref()
    }
}
impl<'a> AstNode<'a, TSTypePredicate<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn parameter_name(&self) -> &AstNode<'a, TSTypePredicateName<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.parameter_name,
            allocator: self.allocator,
            parent: self.parent,
        })
    }

    #[inline]
    pub fn asserts(&self) -> bool {
        self.inner.asserts
    }

    #[inline]
    pub fn type_annotation(&self) -> Option<&AstNode<'a, TSTypeAnnotation<'a>>> {
        self.allocator
            .alloc(self.inner.type_annotation.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.parent,
            }))
            .as_ref()
    }
}

impl<'a> AstNode<'a, TSTypePredicateName<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.parent;
        let node = match self.inner {
            TSTypePredicateName::Identifier(s) => {
                AstNodes::IdentifierName(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            TSTypePredicateName::This(s) => AstNodes::TSThisType(self.allocator.alloc(AstNode {
                inner: s,
                parent,
                allocator: self.allocator,
            })),
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, TSTypePredicateName<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.parent;
        match self.inner {
            TSTypePredicateName::Identifier(inner) => self
                .allocator
                .alloc(AstNode::<IdentifierName> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSTypePredicateName::This(inner) => self
                .allocator
                .alloc(AstNode::<TSThisType> { inner, parent, allocator: self.allocator })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, TSTypePredicateName<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, TSModuleDeclaration<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn id(&self) -> &AstNode<'a, TSModuleDeclarationName<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.id,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSModuleDeclaration(transmute_self(self))),
        })
    }

    #[inline]
    pub fn body(&self) -> Option<&AstNode<'a, TSModuleDeclarationBody<'a>>> {
        self.allocator
            .alloc(self.inner.body.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::TSModuleDeclaration(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn kind(&self) -> TSModuleDeclarationKind {
        self.inner.kind
    }

    #[inline]
    pub fn declare(&self) -> bool {
        self.inner.declare
    }
}

impl<'a> AstNode<'a, TSModuleDeclarationName<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.parent;
        let node = match self.inner {
            TSModuleDeclarationName::Identifier(s) => {
                AstNodes::BindingIdentifier(self.allocator.alloc(AstNode {
                    inner: s,
                    parent,
                    allocator: self.allocator,
                }))
            }
            TSModuleDeclarationName::StringLiteral(s) => {
                AstNodes::StringLiteral(self.allocator.alloc(AstNode {
                    inner: s,
                    parent,
                    allocator: self.allocator,
                }))
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, TSModuleDeclarationName<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.parent;
        match self.inner {
            TSModuleDeclarationName::Identifier(inner) => self
                .allocator
                .alloc(AstNode::<BindingIdentifier> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSModuleDeclarationName::StringLiteral(inner) => self
                .allocator
                .alloc(AstNode::<StringLiteral> { inner, parent, allocator: self.allocator })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, TSModuleDeclarationName<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}

impl<'a> AstNode<'a, TSModuleDeclarationBody<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.parent;
        let node = match self.inner {
            TSModuleDeclarationBody::TSModuleDeclaration(s) => {
                AstNodes::TSModuleDeclaration(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            TSModuleDeclarationBody::TSModuleBlock(s) => {
                AstNodes::TSModuleBlock(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, TSModuleDeclarationBody<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.parent;
        match self.inner {
            TSModuleDeclarationBody::TSModuleDeclaration(inner) => self
                .allocator
                .alloc(AstNode::<TSModuleDeclaration> { inner, parent, allocator: self.allocator })
                .fmt(f),
            TSModuleDeclarationBody::TSModuleBlock(inner) => self
                .allocator
                .alloc(AstNode::<TSModuleBlock> { inner, parent, allocator: self.allocator })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, TSModuleDeclarationBody<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, TSModuleBlock<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn directives(&self) -> &AstNode<'a, Vec<'a, Directive<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.directives,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSModuleBlock(transmute_self(self))),
        })
    }

    #[inline]
    pub fn body(&self) -> &AstNode<'a, Vec<'a, Statement<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.body,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSModuleBlock(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, TSTypeLiteral<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn members(&self) -> &AstNode<'a, Vec<'a, TSSignature<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.members,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSTypeLiteral(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, TSInferType<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn type_parameter(&self) -> &AstNode<'a, TSTypeParameter<'a>> {
        self.allocator.alloc(AstNode {
            inner: self.inner.type_parameter.as_ref(),
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSInferType(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, TSTypeQuery<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn expr_name(&self) -> &AstNode<'a, TSTypeQueryExprName<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.expr_name,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSTypeQuery(transmute_self(self))),
        })
    }

    #[inline]
    pub fn type_arguments(&self) -> Option<&AstNode<'a, TSTypeParameterInstantiation<'a>>> {
        self.allocator
            .alloc(self.inner.type_arguments.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::TSTypeQuery(transmute_self(self))),
            }))
            .as_ref()
    }
}

impl<'a> AstNode<'a, TSTypeQueryExprName<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.parent;
        let node = match self.inner {
            TSTypeQueryExprName::TSImportType(s) => {
                AstNodes::TSImportType(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            it @ match_ts_type_name!(TSTypeQueryExprName) => {
                AstNodes::TSTypeName(self.allocator.alloc(AstNode {
                    inner: it.to_ts_type_name(),
                    parent,
                    allocator: self.allocator,
                }))
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, TSTypeQueryExprName<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.parent;
        match self.inner {
            TSTypeQueryExprName::TSImportType(inner) => self
                .allocator
                .alloc(AstNode::<TSImportType> { inner, parent, allocator: self.allocator })
                .fmt(f),
            it @ match_ts_type_name!(TSTypeQueryExprName) => self
                .allocator
                .alloc(AstNode::<'a, TSTypeName> {
                    inner: it.to_ts_type_name(),
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, TSTypeQueryExprName<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, TSImportType<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn argument(&self) -> &AstNode<'a, TSType<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.argument,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSImportType(transmute_self(self))),
        })
    }

    #[inline]
    pub fn options(&self) -> Option<&AstNode<'a, ObjectExpression<'a>>> {
        self.allocator
            .alloc(self.inner.options.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::TSImportType(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn qualifier(&self) -> Option<&AstNode<'a, TSTypeName<'a>>> {
        self.allocator
            .alloc(self.inner.qualifier.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::TSImportType(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn type_arguments(&self) -> Option<&AstNode<'a, TSTypeParameterInstantiation<'a>>> {
        self.allocator
            .alloc(self.inner.type_arguments.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::TSImportType(transmute_self(self))),
            }))
            .as_ref()
    }
}
impl<'a> AstNode<'a, TSFunctionType<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn type_parameters(&self) -> Option<&AstNode<'a, TSTypeParameterDeclaration<'a>>> {
        self.allocator
            .alloc(self.inner.type_parameters.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.parent,
            }))
            .as_ref()
    }

    #[inline]
    pub fn this_param(&self) -> Option<&AstNode<'a, TSThisParameter<'a>>> {
        self.allocator
            .alloc(self.inner.this_param.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.parent,
            }))
            .as_ref()
    }

    #[inline]
    pub fn params(&self) -> &AstNode<'a, FormalParameters<'a>> {
        self.allocator.alloc(AstNode {
            inner: self.inner.params.as_ref(),
            allocator: self.allocator,
            parent: self.parent,
        })
    }

    #[inline]
    pub fn return_type(&self) -> &AstNode<'a, TSTypeAnnotation<'a>> {
        self.allocator.alloc(AstNode {
            inner: self.inner.return_type.as_ref(),
            allocator: self.allocator,
            parent: self.parent,
        })
    }
}
impl<'a> AstNode<'a, TSConstructorType<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn r#abstract(&self) -> bool {
        self.inner.r#abstract
    }

    #[inline]
    pub fn type_parameters(&self) -> Option<&AstNode<'a, TSTypeParameterDeclaration<'a>>> {
        self.allocator
            .alloc(self.inner.type_parameters.as_ref().map(|inner| AstNode {
                inner: inner.as_ref(),
                allocator: self.allocator,
                parent: self.parent,
            }))
            .as_ref()
    }

    #[inline]
    pub fn params(&self) -> &AstNode<'a, FormalParameters<'a>> {
        self.allocator.alloc(AstNode {
            inner: self.inner.params.as_ref(),
            allocator: self.allocator,
            parent: self.parent,
        })
    }

    #[inline]
    pub fn return_type(&self) -> &AstNode<'a, TSTypeAnnotation<'a>> {
        self.allocator.alloc(AstNode {
            inner: self.inner.return_type.as_ref(),
            allocator: self.allocator,
            parent: self.parent,
        })
    }
}
impl<'a> AstNode<'a, TSMappedType<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn type_parameter(&self) -> &AstNode<'a, TSTypeParameter<'a>> {
        self.allocator.alloc(AstNode {
            inner: self.inner.type_parameter.as_ref(),
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSMappedType(transmute_self(self))),
        })
    }

    #[inline]
    pub fn name_type(&self) -> Option<&AstNode<'a, TSType<'a>>> {
        self.allocator
            .alloc(self.inner.name_type.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::TSMappedType(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn type_annotation(&self) -> Option<&AstNode<'a, TSType<'a>>> {
        self.allocator
            .alloc(self.inner.type_annotation.as_ref().map(|inner| AstNode {
                inner,
                allocator: self.allocator,
                parent: self.allocator.alloc(AstNodes::TSMappedType(transmute_self(self))),
            }))
            .as_ref()
    }

    #[inline]
    pub fn optional(&self) -> Option<TSMappedTypeModifierOperator> {
        self.inner.optional
    }

    #[inline]
    pub fn readonly(&self) -> Option<TSMappedTypeModifierOperator> {
        self.inner.readonly
    }
}
impl<'a> AstNode<'a, TSTemplateLiteralType<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn quasis(&self) -> &AstNode<'a, Vec<'a, TemplateElement<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.quasis,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSTemplateLiteralType(transmute_self(self))),
        })
    }

    #[inline]
    pub fn types(&self) -> &AstNode<'a, Vec<'a, TSType<'a>>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.types,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSTemplateLiteralType(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, TSAsExpression<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn expression(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.expression,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSAsExpression(transmute_self(self))),
        })
    }

    #[inline]
    pub fn type_annotation(&self) -> &AstNode<'a, TSType<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.type_annotation,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSAsExpression(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, TSSatisfiesExpression<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn expression(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.expression,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSSatisfiesExpression(transmute_self(self))),
        })
    }

    #[inline]
    pub fn type_annotation(&self) -> &AstNode<'a, TSType<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.type_annotation,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSSatisfiesExpression(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, TSTypeAssertion<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn type_annotation(&self) -> &AstNode<'a, TSType<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.type_annotation,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSTypeAssertion(transmute_self(self))),
        })
    }

    #[inline]
    pub fn expression(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.expression,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSTypeAssertion(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, TSImportEqualsDeclaration<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn id(&self) -> &AstNode<'a, BindingIdentifier<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.id,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSImportEqualsDeclaration(transmute_self(self))),
        })
    }

    #[inline]
    pub fn module_reference(&self) -> &AstNode<'a, TSModuleReference<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.module_reference,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSImportEqualsDeclaration(transmute_self(self))),
        })
    }

    #[inline]
    pub fn import_kind(&self) -> ImportOrExportKind {
        self.inner.import_kind
    }
}

impl<'a> AstNode<'a, TSModuleReference<'a>> {
    #[inline]
    pub fn as_ast_nodes(&self) -> &AstNodes<'a> {
        let parent = self.allocator.alloc(AstNodes::TSModuleReference(transmute_self(self)));
        let node = match self.inner {
            TSModuleReference::ExternalModuleReference(s) => {
                AstNodes::TSExternalModuleReference(self.allocator.alloc(AstNode {
                    inner: s.as_ref(),
                    parent,
                    allocator: self.allocator,
                }))
            }
            it @ match_ts_type_name!(TSModuleReference) => {
                AstNodes::TSTypeName(self.allocator.alloc(AstNode {
                    inner: it.to_ts_type_name(),
                    parent,
                    allocator: self.allocator,
                }))
            }
        };
        self.allocator.alloc(node)
    }
}
impl<'a> FormatWrite<'a> for AstNode<'a, TSModuleReference<'a>> {
    #[inline]
    fn write(&self, f: &mut Formatter<'_, 'a>) -> FormatResult<()> {
        let parent = self.allocator.alloc(AstNodes::TSModuleReference(transmute_self(self)));
        match self.inner {
            TSModuleReference::ExternalModuleReference(inner) => self
                .allocator
                .alloc(AstNode::<TSExternalModuleReference> {
                    inner,
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
            it @ match_ts_type_name!(TSModuleReference) => self
                .allocator
                .alloc(AstNode::<'a, TSTypeName> {
                    inner: it.to_ts_type_name(),
                    parent,
                    allocator: self.allocator,
                })
                .fmt(f),
        }
    }
}
impl<'a> GetSpan for AstNode<'a, TSModuleReference<'a>> {
    #[inline]
    fn span(&self) -> oxc_span::Span {
        self.inner.span()
    }
}
impl<'a> AstNode<'a, TSExternalModuleReference<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn expression(&self) -> &AstNode<'a, StringLiteral<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.expression,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSExternalModuleReference(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, TSNonNullExpression<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn expression(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.expression,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSNonNullExpression(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, Decorator<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn expression(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.expression,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::Decorator(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, TSExportAssignment<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn expression(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.expression,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSExportAssignment(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, TSNamespaceExportDeclaration<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn id(&self) -> &AstNode<'a, IdentifierName<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.id,
            allocator: self.allocator,
            parent: self.parent,
        })
    }
}
impl<'a> AstNode<'a, TSInstantiationExpression<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn expression(&self) -> &AstNode<'a, Expression<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.expression,
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSInstantiationExpression(transmute_self(self))),
        })
    }

    #[inline]
    pub fn type_arguments(&self) -> &AstNode<'a, TSTypeParameterInstantiation<'a>> {
        self.allocator.alloc(AstNode {
            inner: self.inner.type_arguments.as_ref(),
            allocator: self.allocator,
            parent: self.allocator.alloc(AstNodes::TSInstantiationExpression(transmute_self(self))),
        })
    }
}
impl<'a> AstNode<'a, JSDocNullableType<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn type_annotation(&self) -> &AstNode<'a, TSType<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.type_annotation,
            allocator: self.allocator,
            parent: self.parent,
        })
    }

    #[inline]
    pub fn postfix(&self) -> bool {
        self.inner.postfix
    }
}
impl<'a> AstNode<'a, JSDocNonNullableType<'a>> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }

    #[inline]
    pub fn type_annotation(&self) -> &AstNode<'a, TSType<'a>> {
        self.allocator.alloc(AstNode {
            inner: &self.inner.type_annotation,
            allocator: self.allocator,
            parent: self.parent,
        })
    }

    #[inline]
    pub fn postfix(&self) -> bool {
        self.inner.postfix
    }
}
impl<'a> AstNode<'a, JSDocUnknownType> {
    #[inline]
    pub fn span(&self) -> Span {
        self.inner.span
    }
}
