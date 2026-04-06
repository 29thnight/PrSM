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
