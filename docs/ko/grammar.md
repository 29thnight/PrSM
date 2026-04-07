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

## PrSM 5 문법 추가 (PrSM 5 부터)

언어 5는 위 문법을 다음 production으로 확장합니다.

### 선언 한정자

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

### 매개변수

```ebnf
Param           = [ ParamMod ] IDENT ":" TypeRef [ "=" DefaultExpr ] ;

ParamMod        = "ref" | "out" | "vararg" ;

DefaultExpr     = LiteralExpr | NullLiteral | "default" ;
```

### 코루틴과 yield

```ebnf
YieldStmt       = "yield" Expr NEWLINE
                | "yield" "break" NEWLINE
                | "yield" "return" Expr NEWLINE ;

Statement       = ... | YieldStmt ;
```

### 변수 선언

```ebnf
VarDecl         = ( "val" | "var" ) [ "ref" ] IDENT [ ":" TypeRef ]
                  "=" [ "ref" ] Expr NEWLINE ;
```

### 타입 참조

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

### 어트리뷰트 타깃

```ebnf
AttrTargetDecl  = "@" AttrTarget "(" AttrName [ "," AttrArgs ] ")" NEWLINE ;
AttrTarget      = "field" | "property" | "param" | "return" | "type" ;
```

### 전처리

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

### 패턴

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

### 식

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
