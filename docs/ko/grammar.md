---
title: 형식 문법
parent: 언어 가이드
grand_parent: 한국어 문서
nav_order: 1
---

# 형식 문법

이 페이지는 PrSM 표면 구문을 확장 배커스-나우르 형식(EBNF)으로 정의합니다. 이 문법은 파서의 권위 있는 참조이며, 다른 페이지의 설명 문서는 이 문법에서 파생됩니다.

표기 규칙: `=` 정의, `|` 대안, `[ ]` 선택(생략 가능), `{ }` 반복(0회 이상), `( )` 그룹화, `"keyword"` 터미널 키워드 또는 기호, `UPPER_CASE` 터미널 토큰 클래스.

---

## 파일 구조

```ebnf
File          = { UsingDecl } Declaration ;

UsingDecl     = "using" QualifiedName NEWLINE ;

QualifiedName = IDENT { "." IDENT } ;
```

## 선언

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

## 멤버

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

## 라이프사이클 블록

```ebnf
LifecycleBlock  = LifecycleName [ LifecycleParam ] Block ;

LifecycleName   = "awake" | "start" | "update" | "fixedUpdate" | "lateUpdate"
                | "onEnable" | "onDisable" | "onDestroy"
                | "onTriggerEnter" | "onTriggerExit" | "onTriggerStay"
                | "onCollisionEnter" | "onCollisionExit" | "onCollisionStay" ;

LifecycleParam  = "(" IDENT ":" TypeRef ")" ;
```

## 함수와 코루틴

```ebnf
FuncDecl        = [ VisibilityMod ] "func" IDENT "(" [ ParamList ] ")" [ ":" TypeRef ]
                  ( Block | "=" Expr NEWLINE ) ;

CoroutineDecl   = "coroutine" IDENT "(" [ ParamList ] ")" Block ;

VisibilityMod   = "public" | "private" ;

ParamList       = Param { "," Param } ;

Param           = IDENT ":" TypeRef [ "=" Expr ] ;
```

## listen과 intrinsic

```ebnf
ListenDecl      = "listen" Expr [ ListenLifetime ] [ LambdaParam ] Block ;

ListenLifetime  = "." ( "once" | "whileEnabled" ) ;

LambdaParam     = "{" "val" IDENT "->" ;

UnlistenStmt    = "unlisten" Expr NEWLINE ;

IntrinsicBlock  = "intrinsic" ( Block | "(" TypeRef ")" Block ) ;
```

## 어노테이션

```ebnf
Annotation      = "@" IDENT [ "(" AnnotationArgs ")" ] NEWLINE ;

AnnotationArgs  = AnnotationArg { "," AnnotationArg } ;

AnnotationArg   = [ IDENT "=" ] Expr ;
```

## 문(Statement)

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

## 패턴

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

## 식(Expression) (우선순위: 낮음 -> 높음)

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

## 타입 참조

```ebnf
TypeRef         = QualifiedName [ "<" TypeRefList ">" ] [ "?" ] ;

TypeRefList     = TypeRef { "," TypeRef } ;
```

## 터미널 토큰

```ebnf
IDENT           = LETTER { LETTER | DIGIT | "_" } ;
INT_LIT         = DIGIT { DIGIT } ;
FLOAT_LIT       = DIGIT { DIGIT } "." DIGIT { DIGIT } [ "f" ] ;
STRING_LIT      = '"' { CHAR | "$" IDENT | "${" Expr "}" } '"' ;
NEWLINE         = "\n" ;
LETTER          = "a".."z" | "A".."Z" | "_" ;
DIGIT           = "0".."9" ;
```
