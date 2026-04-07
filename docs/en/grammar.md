---
title: Formal Grammar
parent: Language Guide
grand_parent: English Docs
nav_order: 1
---

# Formal Grammar

This page defines the PrSM surface syntax in Extended Backus-Naur Form (EBNF). The grammar is the authoritative reference for the parser; prose documentation in other pages is derived from it.

Conventions: `=` definition, `|` alternative, `[ ]` optional, `{ }` repetition (zero or more), `( )` grouping, `"keyword"` terminal keyword or symbol, `UPPER_CASE` terminal token class.

---

## File structure

```ebnf
File          = { UsingDecl } Declaration ;

UsingDecl     = "using" QualifiedName NEWLINE ;

QualifiedName = IDENT { "." IDENT } ;
```

## Declarations

```ebnf
Declaration   = ComponentDecl
              | AssetDecl
              | ClassDecl
              | DataClassDecl
              | EnumDecl
              | AttributeDecl ;

ComponentDecl = "component" IDENT [ ":" TypeRef ] "{" { ComponentMember } "}" ;

AssetDecl     = "asset" IDENT [ ":" TypeRef ] "{" { AssetMember } "}" ;

ClassDecl     = "class" IDENT [ ":" TypeRef ] "{" { ClassMember } "}" ;

DataClassDecl = "data" "class" IDENT "(" ParamList ")" ;

EnumDecl      = "enum" IDENT "{" EnumEntry { "," EnumEntry } [ "," ] "}" ;

EnumEntry     = IDENT [ "(" TypeRefList ")" ] ;

AttributeDecl = "attribute" IDENT [ "(" ParamList ")" ] "{" { ClassMember } "}" ;
```

## Members

```ebnf
ComponentMember = FieldDecl
                | LifecycleBlock
                | FuncDecl
                | CoroutineDecl
                | ListenDecl
                | IntrinsicBlock ;

AssetMember     = FieldDecl
                | FuncDecl
                | IntrinsicBlock ;

ClassMember     = FieldDecl
                | FuncDecl
                | IntrinsicBlock ;

FieldDecl       = { Annotation } FieldKind IDENT ":" TypeRef [ "=" Expr ] NEWLINE ;

FieldKind       = "serialize"
                | "require"
                | "optional"
                | "child"
                | "parent"
                | "val"
                | "var" ;
```

## Lifecycle blocks

```ebnf
LifecycleBlock  = LifecycleName [ LifecycleParam ] Block ;

LifecycleName   = "awake" | "start" | "update" | "fixedUpdate" | "lateUpdate"
                | "onEnable" | "onDisable" | "onDestroy"
                | "onTriggerEnter" | "onTriggerExit" | "onTriggerStay"
                | "onCollisionEnter" | "onCollisionExit" | "onCollisionStay" ;

LifecycleParam  = "(" IDENT ":" TypeRef ")" ;
```

## Functions and coroutines

```ebnf
FuncDecl        = [ VisibilityMod ] "func" IDENT "(" [ ParamList ] ")" [ ":" TypeRef ]
                  ( Block | "=" Expr NEWLINE ) ;

CoroutineDecl   = "coroutine" IDENT "(" [ ParamList ] ")" Block ;

VisibilityMod   = "public" | "private" ;

ParamList       = Param { "," Param } ;

Param           = IDENT ":" TypeRef [ "=" Expr ] ;
```

## Listen and intrinsic

```ebnf
ListenDecl      = "listen" Expr [ ListenLifetime ] [ LambdaParam ] Block ;

ListenLifetime  = "." ( "once" | "whileEnabled" ) ;

LambdaParam     = "{" "val" IDENT "->" ;

UnlistenStmt    = "unlisten" Expr NEWLINE ;

IntrinsicBlock  = "intrinsic" ( Block | "(" TypeRef ")" Block ) ;
```

## Annotations

```ebnf
Annotation      = "@" IDENT [ "(" AnnotationArgs ")" ] NEWLINE ;

AnnotationArgs  = AnnotationArg { "," AnnotationArg } ;

AnnotationArg   = [ IDENT "=" ] Expr ;
```

## Statements

```ebnf
Block           = "{" { Statement } "}" ;

Statement       = VarDecl
                | Assignment
                | IfStmt
                | WhenStmt
                | ForStmt
                | WhileStmt
                | ReturnStmt
                | WaitStmt
                | BreakStmt
                | ContinueStmt
                | UnlistenStmt
                | IntrinsicBlock
                | ExprStmt ;

VarDecl         = ( "val" | "var" ) IDENT [ ":" TypeRef ] "=" Expr NEWLINE ;

Assignment      = Expr AssignOp Expr NEWLINE ;

AssignOp        = "=" | "+=" | "-=" | "*=" | "/=" | "%=" ;

IfStmt          = "if" Expr Block { "else" "if" Expr Block } [ "else" Block ] ;

WhenStmt        = "when" [ Expr ] "{" { WhenBranch } [ ElseBranch ] "}" ;

WhenBranch      = Pattern "=>" ( Expr NEWLINE | Block ) ;

ElseBranch      = "else" "=>" ( Expr NEWLINE | Block ) ;

ForStmt         = "for" IDENT "in" Expr Block ;

WhileStmt       = "while" Expr Block ;

ReturnStmt      = "return" [ Expr ] NEWLINE ;

WaitStmt        = "wait" WaitForm NEWLINE ;

WaitForm        = Expr "." "s"
                | Expr "s"
                | "nextFrame"
                | "fixedFrame"
                | "until" Expr
                | "while" Expr ;

BreakStmt       = "break" NEWLINE ;

ContinueStmt   = "continue" NEWLINE ;

ExprStmt        = Expr NEWLINE ;
```

## Patterns

```ebnf
Pattern         = LiteralPattern
                | EnumPattern
                | BindingPattern
                | DestructurePattern
                | WildcardPattern ;

LiteralPattern  = INT_LIT | FLOAT_LIT | STRING_LIT | "true" | "false" | "null" ;

EnumPattern     = QualifiedName [ "(" PatternList ")" ] ;

BindingPattern  = "val" IDENT ;

DestructurePattern = QualifiedName "(" PatternList ")" ;

WildcardPattern = "_" ;

PatternList     = Pattern { "," Pattern } ;
```

## Expressions (precedence low to high)

```ebnf
Expr            = OrExpr ;

OrExpr          = AndExpr { "||" AndExpr } ;

AndExpr         = EqualityExpr { "&&" EqualityExpr } ;

EqualityExpr    = ComparisonExpr { ( "==" | "!=" ) ComparisonExpr } ;

ComparisonExpr  = RangeExpr { ( "<" | ">" | "<=" | ">=" ) RangeExpr } ;

RangeExpr       = AdditiveExpr [ ( ".." | "until" | "downTo" ) AdditiveExpr [ "step" AdditiveExpr ] ] ;

AdditiveExpr    = MultExpr { ( "+" | "-" ) MultExpr } ;

MultExpr        = UnaryExpr { ( "*" | "/" | "%" ) UnaryExpr } ;

UnaryExpr       = ( "-" | "!" ) UnaryExpr
                | PostfixExpr ;

PostfixExpr     = PrimaryExpr { PostfixOp } ;

PostfixOp       = "." IDENT
                | "?." IDENT
                | "!!"
                | "(" [ ArgList ] ")"
                | "[" Expr "]"
                | "is" TypeRef ;

PrimaryExpr     = IDENT
                | INT_LIT | FLOAT_LIT | STRING_LIT
                | "true" | "false" | "null"
                | "this"
                | "(" Expr ")"
                | IfExpr
                | WhenExpr
                | "?:" Expr ;

IfExpr          = "if" Expr Block "else" Block ;

WhenExpr        = "when" [ Expr ] "{" { WhenBranch } [ ElseBranch ] "}" ;

ArgList         = Arg { "," Arg } ;

Arg             = [ IDENT ":" ] Expr ;
```

## Type references

```ebnf
TypeRef         = QualifiedName [ "<" TypeRefList ">" ] [ "?" ] ;

TypeRefList     = TypeRef { "," TypeRef } ;
```

## Terminal tokens

```ebnf
IDENT           = LETTER { LETTER | DIGIT | "_" } ;
INT_LIT         = DIGIT { DIGIT } ;
FLOAT_LIT       = DIGIT { DIGIT } "." DIGIT { DIGIT } [ "f" ] ;
STRING_LIT      = '"' { CHAR | "$" IDENT | "${" Expr "}" } '"' ;
NEWLINE         = "\n" ;
LETTER          = "a".."z" | "A".."Z" | "_" ;
DIGIT           = "0".."9" ;
```

## PrSM 5 grammar additions (since PrSM 5)

Language 5 extends the grammar above with the following productions.

### Declaration modifiers

```ebnf
ComponentDecl   = { Annotation } [ "partial" ] [ "singleton" ] "component" IDENT
                  [ ":" TypeRef ] "{" { ComponentMember } "}" ;

ClassDecl       = { Annotation } [ "partial" ] [ ClassMod ] "class" IDENT
                  [ ":" TypeRef ] "{" { ClassMember } "}" ;

StructDecl      = { Annotation } [ "partial" ] [ "ref" ] "struct" IDENT
                  "(" ParamList ")" [ "{" { ClassMember } "}" ] ;

NestedDecl      = ClassDecl | StructDecl | EnumDecl | DataClassDecl | InterfaceDecl ;

ComponentMember = ... | NestedDecl ;
ClassMember     = ... | NestedDecl ;
```

### Parameters

```ebnf
Param           = [ ParamMod ] IDENT ":" TypeRef [ "=" DefaultExpr ] ;

ParamMod        = "ref" | "out" | "vararg" ;

DefaultExpr     = LiteralExpr | NullLiteral | "default" ;
```

### Coroutine and yield

```ebnf
YieldStmt       = "yield" Expr NEWLINE
                | "yield" "break" NEWLINE
                | "yield" "return" Expr NEWLINE ;

Statement       = ... | YieldStmt ;
```

### Variable declarations

```ebnf
VarDecl         = ( "val" | "var" ) [ "ref" ] IDENT [ ":" TypeRef ]
                  "=" [ "ref" ] Expr NEWLINE ;
```

### Type references

```ebnf
TypeRef         = [ "ref" ] QualifiedName [ "<" TypeRefList ">" ] [ "?" ] ;

WhereConstraint = TypeRef
                | "class"
                | "struct"
                | "unmanaged"
                | "notnull"
                | "default"
                | "new" "(" ")" ;
```

### Attribute targets

```ebnf
AttrTargetDecl  = "@" AttrTarget "(" AttrName [ "," AttrArgs ] ")" NEWLINE ;
AttrTarget      = "field" | "property" | "param" | "return" | "type" ;
```

### Preprocessor

```ebnf
IfDirective     = "#if" Condition Block { ElseIfDirective } [ ElseDirective ] "#endif" ;
ElseIfDirective = "#elif" Condition Block ;
ElseDirective   = "#else" Block ;
Condition       = SymbolName
                | SymbolName "(" Args ")"
                | "!" Condition
                | Condition "&&" Condition
                | Condition "||" Condition
                | "(" Condition ")" ;
SymbolName      = "editor" | "debug" | "release" | "ios" | "android"
                | "standalone" | "il2cpp" | "mono"
                | "unity20223" | "unity20231" | "unity6"
                | IDENT ;
```

### Patterns

```ebnf
Pattern         = OrPattern ;
OrPattern       = AndPattern { "or" AndPattern } ;
AndPattern      = NotPattern { "and" NotPattern } ;
NotPattern      = [ "not" ] PrimaryPattern ;

PrimaryPattern  = LiteralPattern
                | RelationalPattern
                | EnumPattern
                | PositionalPattern
                | PropertyPattern
                | BindingPattern
                | DiscardPattern ;

RelationalPattern = ( "<" | ">" | "<=" | ">=" ) Expr ;

PositionalPattern = TypeName "(" [ Pattern { "," Pattern } ] ")" ;

PropertyPattern   = [ TypeName ] "{" [ PropPatternEntry { "," PropPatternEntry } ] "}" ;

PropPatternEntry  = IDENT ":" Pattern ;

DiscardPattern    = "_" ;
```

### Expressions

```ebnf
PostfixOp       = ... | "?[" Expr "]" ;

PrimaryExpr     = ... | NameOfExpr | StackallocExpr | ThrowExpr | DiscardExpr | WithExpr ;

NameOfExpr      = "nameof" "(" QualifiedIdent ")" ;

StackallocExpr  = "stackalloc" "[" TypeRef "]" "(" Expr ")" ;

ThrowExpr       = "throw" Expr ;

DiscardExpr     = "_" ;

WithExpr        = Expr "with" "{" FieldAssign { "," FieldAssign } "}" ;

FieldAssign     = IDENT "=" Expr ;

Arg             = [ IDENT ":" ] [ "ref" | "out" ] Expr
                | "out" "val" IDENT
                | "out" "var" IDENT
                | "out" "_" ;
```

### Async backend

```ebnf
ProjectAsync    = "[" "language" "." "async" "]" NEWLINE
                  "backend" "=" ( "\"unitask\"" | "\"task\"" | "\"auto\"" ) ;
```
