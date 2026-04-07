use serde::Serialize;

/// Source position for diagnostics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Position {
    pub line: u32,
    pub col: u32,
}

/// Source span — a range in a source file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new(start_line: u32, start_col: u32, end_line: u32, end_col: u32) -> Self {
        Span {
            start: Position { line: start_line, col: start_col },
            end: Position { line: end_line, col: end_col },
        }
    }
}

/// A single token produced by the lexer.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Token { kind, span }
    }
}

/// All token kinds in the PrSM language (v1).
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // === Literals ===
    IntLiteral(i64),
    FloatLiteral(f64),
    DurationLiteral(f64),      // e.g., 1.0s → seconds as f64
    BoolTrue,
    BoolFalse,

    // === Identifiers ===
    Identifier(String),

    // === String literal (simple, no interpolation) ===
    StringLiteral(String),

    // === String interpolation parts ===
    /// Opening `"` text before first `${` or `$ident`
    StringStart(String),
    /// Text between interpolation segments
    StringMiddle(String),
    /// Closing text after last interpolation through `"`
    StringEnd(String),
    /// Signals start of `${` interpolation expression
    InterpolationExprStart,
    /// Signals end of `}` for interpolation expression
    InterpolationExprEnd,
    /// Simple `$identifier` interpolation
    InterpolationIdent(String),

    // === Keywords: declarations ===
    Component,
    Asset,
    Class,
    Data,           // context keyword: "data class"
    Enum,

    // === Keywords: attribute, interface, singleton ===
    Attribute,
    Interface,
    Singleton,

    // === Keywords: field qualifiers ===
    Serialize,
    Require,
    Optional,
    Child,
    Parent,
    Pool,

    // === Keywords: mutability ===
    Val,
    Var,
    Const,
    Fixed,

    // === Keywords: visibility ===
    Public,
    Private,
    Protected,

    // === Keywords: functions ===
    Func,
    Override,
    Return,

    // === Keywords: coroutine ===
    Coroutine,
    Wait,
    Start,
    Stop,
    StopAll,

    // === Keywords: lifecycle ===
    Awake,
    Update,
    FixedUpdate,
    LateUpdate,
    OnEnable,
    OnDisable,
    OnDestroy,
    OnTriggerEnter,
    OnTriggerExit,
    OnTriggerStay,
    OnCollisionEnter,
    OnCollisionExit,
    OnCollisionStay,

    // === Keywords: control flow ===
    If,
    Else,
    When,
    For,
    While,
    In,
    Until,
    DownTo,
    Step,
    Break,
    Continue,
    Is,

    // === Keywords: event ===
    Listen,
    Unlisten,
    Manual,

    // === Keywords: intrinsic ===
    Intrinsic,

    // === Keywords: exception handling ===
    Try,
    Catch,
    Finally,
    Throw,

    // === Keywords: modifiers ===
    Static,
    Abstract,
    Sealed,
    Open,

    // === Keywords: type alias ===
    TypeAlias,

    // === Keywords: casting ===
    As,

    // === Keywords: value types ===
    Struct,

    // === Keywords: extensions ===
    Extend,

    // === Keywords: operator ===
    Operator,

    // === Keywords: property accessors ===
    Get,
    Set,
    Field,

    // === Keywords: other ===
    Using,
    Null,
    This,

    // === Keywords: wait modifiers ===
    NextFrame,
    FixedFrame,

    // === Operators ===
    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    Percent,        // %
    EqEq,           // ==
    NotEq,          // !=
    Lt,             // <
    Gt,             // >
    LtEq,           // <=
    GtEq,           // >=
    AmpAmp,         // &&
    PipePipe,       // ||
    Bang,           // !
    BangBang,       // !!
    Eq,             // =
    PlusEq,         // +=
    MinusEq,        // -=
    StarEq,         // *=
    SlashEq,        // /=
    PercentEq,      // %=
    FatArrow,       // =>
    Dot,            // .
    QuestionDot,    // ?.
    Elvis,          // ?:
    ElvisAssign,    // ?:=
    Question,       // ?
    Colon,          // :
    DotDot,         // ..

    // === Delimiters ===
    LParen,         // (
    RParen,         // )
    LBrace,         // {
    RBrace,         // }
    LBracket,       // [
    RBracket,       // ]
    Comma,          // ,
    Semicolon,      // ;
    At,             // @

    // === Special ===
    Newline,
    Eof,

    // === Language 5, Sprint 1: preprocessor directives ===
    /// `#if` — opens a preprocessor block.
    HashIf,
    /// `#elif` — alternative branch.
    HashElif,
    /// `#else` — final alternative branch.
    HashElse,
    /// `#endif` — closes a preprocessor block.
    HashEndif,

    // === Error ===
    Error(String),
}

impl TokenKind {
    /// Returns true if this token kind is a keyword.
    pub fn is_keyword(&self) -> bool {
        matches!(
            self,
            TokenKind::Component
                | TokenKind::Asset
                | TokenKind::Class
                | TokenKind::Data
                | TokenKind::Enum
                | TokenKind::Attribute
                | TokenKind::Interface
                | TokenKind::Singleton
                | TokenKind::Serialize
                | TokenKind::Require
                | TokenKind::Optional
                | TokenKind::Child
                | TokenKind::Parent
                | TokenKind::Pool
                | TokenKind::Val
                | TokenKind::Var
                | TokenKind::Public
                | TokenKind::Private
                | TokenKind::Protected
                | TokenKind::Func
                | TokenKind::Override
                | TokenKind::Return
                | TokenKind::Coroutine
                | TokenKind::Wait
                | TokenKind::Start
                | TokenKind::Stop
                | TokenKind::StopAll
                | TokenKind::Awake
                | TokenKind::Update
                | TokenKind::FixedUpdate
                | TokenKind::LateUpdate
                | TokenKind::OnEnable
                | TokenKind::OnDisable
                | TokenKind::OnDestroy
                | TokenKind::OnTriggerEnter
                | TokenKind::OnTriggerExit
                | TokenKind::OnTriggerStay
                | TokenKind::OnCollisionEnter
                | TokenKind::OnCollisionExit
                | TokenKind::OnCollisionStay
                | TokenKind::If
                | TokenKind::Else
                | TokenKind::When
                | TokenKind::For
                | TokenKind::While
                | TokenKind::In
                | TokenKind::Until
                | TokenKind::DownTo
                | TokenKind::Step
                | TokenKind::Break
                | TokenKind::Continue
                | TokenKind::Is
                | TokenKind::Listen
                | TokenKind::Unlisten
                | TokenKind::Manual
                | TokenKind::Intrinsic
                | TokenKind::Try
                | TokenKind::Catch
                | TokenKind::Finally
                | TokenKind::Throw
                | TokenKind::Static
                | TokenKind::Abstract
                | TokenKind::Sealed
                | TokenKind::Open
                | TokenKind::TypeAlias
                | TokenKind::As
                | TokenKind::Struct
                | TokenKind::Extend
                | TokenKind::Operator
                | TokenKind::Using
                | TokenKind::Null
                | TokenKind::This
                | TokenKind::BoolTrue
                | TokenKind::BoolFalse
                | TokenKind::NextFrame
                | TokenKind::FixedFrame
        )
    }
}

impl TokenKind {
    /// Inverse of `lookup_keyword`: return the raw source text for a
    /// keyword token. Used by `expect_ident_or_keyword` in the parser
    /// when a keyword needs to be accepted in identifier position
    /// (parameter names, state machine state names, etc.). Returns
    /// `None` for non-keyword tokens.
    pub fn keyword_text(&self) -> Option<&'static str> {
        match self {
            // Declarations
            TokenKind::Component => Some("component"),
            TokenKind::Asset => Some("asset"),
            TokenKind::Class => Some("class"),
            TokenKind::Data => Some("data"),
            TokenKind::Enum => Some("enum"),
            TokenKind::Attribute => Some("attribute"),
            TokenKind::Interface => Some("interface"),
            TokenKind::Singleton => Some("singleton"),
            TokenKind::Serialize => Some("serialize"),
            TokenKind::Require => Some("require"),
            TokenKind::Optional => Some("optional"),
            TokenKind::Child => Some("child"),
            TokenKind::Parent => Some("parent"),
            TokenKind::Pool => Some("pool"),
            TokenKind::Val => Some("val"),
            TokenKind::Var => Some("var"),
            TokenKind::Const => Some("const"),
            TokenKind::Fixed => Some("fixed"),
            TokenKind::Public => Some("public"),
            TokenKind::Private => Some("private"),
            TokenKind::Protected => Some("protected"),
            TokenKind::Func => Some("func"),
            TokenKind::Override => Some("override"),
            TokenKind::Return => Some("return"),
            TokenKind::Coroutine => Some("coroutine"),
            // Lifecycle / coroutine keywords
            TokenKind::Wait => Some("wait"),
            TokenKind::Start => Some("start"),
            TokenKind::Stop => Some("stop"),
            TokenKind::StopAll => Some("stopAll"),
            TokenKind::Awake => Some("awake"),
            TokenKind::Update => Some("update"),
            TokenKind::FixedUpdate => Some("fixedUpdate"),
            TokenKind::LateUpdate => Some("lateUpdate"),
            TokenKind::OnEnable => Some("onEnable"),
            TokenKind::OnDisable => Some("onDisable"),
            TokenKind::OnDestroy => Some("onDestroy"),
            TokenKind::OnTriggerEnter => Some("onTriggerEnter"),
            TokenKind::OnTriggerExit => Some("onTriggerExit"),
            TokenKind::OnTriggerStay => Some("onTriggerStay"),
            TokenKind::OnCollisionEnter => Some("onCollisionEnter"),
            TokenKind::OnCollisionExit => Some("onCollisionExit"),
            TokenKind::OnCollisionStay => Some("onCollisionStay"),
            // Control flow
            TokenKind::If => Some("if"),
            TokenKind::Else => Some("else"),
            TokenKind::When => Some("when"),
            TokenKind::For => Some("for"),
            TokenKind::While => Some("while"),
            TokenKind::In => Some("in"),
            TokenKind::Until => Some("until"),
            TokenKind::DownTo => Some("downTo"),
            TokenKind::Step => Some("step"),
            TokenKind::Break => Some("break"),
            TokenKind::Continue => Some("continue"),
            TokenKind::Is => Some("is"),
            // Listen / intrinsic
            TokenKind::Listen => Some("listen"),
            TokenKind::Unlisten => Some("unlisten"),
            TokenKind::Manual => Some("manual"),
            TokenKind::Intrinsic => Some("intrinsic"),
            // Exceptions
            TokenKind::Try => Some("try"),
            TokenKind::Catch => Some("catch"),
            TokenKind::Finally => Some("finally"),
            TokenKind::Throw => Some("throw"),
            // Modifiers
            TokenKind::Static => Some("static"),
            TokenKind::Abstract => Some("abstract"),
            TokenKind::Sealed => Some("sealed"),
            TokenKind::Open => Some("open"),
            TokenKind::TypeAlias => Some("typealias"),
            TokenKind::As => Some("as"),
            TokenKind::Struct => Some("struct"),
            TokenKind::Extend => Some("extend"),
            TokenKind::Operator => Some("operator"),
            TokenKind::Using => Some("using"),
            TokenKind::Null => Some("null"),
            TokenKind::This => Some("this"),
            TokenKind::BoolTrue => Some("true"),
            TokenKind::BoolFalse => Some("false"),
            TokenKind::NextFrame => Some("nextFrame"),
            TokenKind::FixedFrame => Some("fixedFrame"),
            _ => None,
        }
    }
}

/// Look up an identifier string to see if it's a keyword.
pub fn lookup_keyword(ident: &str) -> Option<TokenKind> {
    match ident {
        "component" => Some(TokenKind::Component),
        "asset" => Some(TokenKind::Asset),
        "class" => Some(TokenKind::Class),
        "data" => Some(TokenKind::Data),
        "enum" => Some(TokenKind::Enum),
        "attribute" => Some(TokenKind::Attribute),
        "interface" => Some(TokenKind::Interface),
        "singleton" => Some(TokenKind::Singleton),
        "serialize" => Some(TokenKind::Serialize),
        "require" => Some(TokenKind::Require),
        "optional" => Some(TokenKind::Optional),
        "child" => Some(TokenKind::Child),
        "parent" => Some(TokenKind::Parent),
        "pool" => Some(TokenKind::Pool),
        "val" => Some(TokenKind::Val),
        "var" => Some(TokenKind::Var),
        "const" => Some(TokenKind::Const),
        "fixed" => Some(TokenKind::Fixed),
        "public" => Some(TokenKind::Public),
        "private" => Some(TokenKind::Private),
        "protected" => Some(TokenKind::Protected),
        "func" => Some(TokenKind::Func),
        "override" => Some(TokenKind::Override),
        "return" => Some(TokenKind::Return),
        "coroutine" => Some(TokenKind::Coroutine),
        "wait" => Some(TokenKind::Wait),
        "start" => Some(TokenKind::Start),
        "stop" => Some(TokenKind::Stop),
        "stopAll" => Some(TokenKind::StopAll),
        "awake" => Some(TokenKind::Awake),
        "update" => Some(TokenKind::Update),
        "fixedUpdate" => Some(TokenKind::FixedUpdate),
        "lateUpdate" => Some(TokenKind::LateUpdate),
        "onEnable" => Some(TokenKind::OnEnable),
        "onDisable" => Some(TokenKind::OnDisable),
        "onDestroy" => Some(TokenKind::OnDestroy),
        "onTriggerEnter" => Some(TokenKind::OnTriggerEnter),
        "onTriggerExit" => Some(TokenKind::OnTriggerExit),
        "onTriggerStay" => Some(TokenKind::OnTriggerStay),
        "onCollisionEnter" => Some(TokenKind::OnCollisionEnter),
        "onCollisionExit" => Some(TokenKind::OnCollisionExit),
        "onCollisionStay" => Some(TokenKind::OnCollisionStay),
        "if" => Some(TokenKind::If),
        "else" => Some(TokenKind::Else),
        "when" => Some(TokenKind::When),
        "for" => Some(TokenKind::For),
        "while" => Some(TokenKind::While),
        "in" => Some(TokenKind::In),
        "until" => Some(TokenKind::Until),
        "downTo" => Some(TokenKind::DownTo),
        "step" => Some(TokenKind::Step),
        "break" => Some(TokenKind::Break),
        "continue" => Some(TokenKind::Continue),
        "is" => Some(TokenKind::Is),
        "listen" => Some(TokenKind::Listen),
        "unlisten" => Some(TokenKind::Unlisten),
        "manual" => Some(TokenKind::Manual),
        "intrinsic" => Some(TokenKind::Intrinsic),
        "try" => Some(TokenKind::Try),
        "catch" => Some(TokenKind::Catch),
        "finally" => Some(TokenKind::Finally),
        "throw" => Some(TokenKind::Throw),
        "static" => Some(TokenKind::Static),
        "abstract" => Some(TokenKind::Abstract),
        "sealed" => Some(TokenKind::Sealed),
        "open" => Some(TokenKind::Open),
        "typealias" => Some(TokenKind::TypeAlias),
        "as" => Some(TokenKind::As),
        "struct" => Some(TokenKind::Struct),
        "extend" => Some(TokenKind::Extend),
        "operator" => Some(TokenKind::Operator),
        // Note: `get`, `set`, and `field` are contextual keywords. They are
        // produced as `Identifier` here so that existing usages such as the
        // built-in `get<Component>()` sugar continue to work, and the parser
        // recognises them by their identifier text inside property accessors.
        "using" => Some(TokenKind::Using),
        "null" => Some(TokenKind::Null),
        "this" => Some(TokenKind::This),
        "true" => Some(TokenKind::BoolTrue),
        "false" => Some(TokenKind::BoolFalse),
        "nextFrame" => Some(TokenKind::NextFrame),
        "fixedFrame" => Some(TokenKind::FixedFrame),
        _ => None,
    }
}
