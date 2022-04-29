/// 位置情報 .0から.1までの区間を表す
/// ex. Loc(4, 6)：5文字目から7文字目を表す（0始まり）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Loc(usize, usize);

impl Loc {
    fn merge(&self, other: &Loc) -> Loc {
        use std::cmp::{max, min};
        Loc(min(self.0, other.0), max(self.1, other.1))
    }
}

/// アノテーション　値にLocをもたせたもの
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Annot<T> {
    value: T,
    loc: Loc,
}

impl<T> Annot<T> {
    fn new(value: T, loc: Loc) -> Self {
        Self { value, loc }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum TokenKind {
    /// [0-9][0-9]*
    Number(u64),
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Asterisk,
    /// /
    Slash,
    /// (
    LParen,
    /// )
    RParen,
}

// Tokenアノテーション
type Token = Annot<TokenKind>;

// ヘルパーメソッド
impl Token {
    fn number(n: u64, loc: Loc) -> Self {
        Self::new(TokenKind::Number(n), loc)
    }
    fn plus(loc: Loc) -> Self {
        Self::new(TokenKind::Plus, loc)
    }
    fn minus(loc: Loc) -> Self {
        Self::new(TokenKind::Minus, loc)
    }
    fn asterisk(loc: Loc) -> Self {
        Self::new(TokenKind::Asterisk, loc)
    }
    fn slash(loc: Loc) -> Self {
        Self::new(TokenKind::Slash, loc)
    }
    fn lparen(loc: Loc) -> Self {
        Self::new(TokenKind::LParen, loc)
    }
    fn rparen(loc: Loc) -> Self {
        Self::new(TokenKind::RParen, loc)
    }
}

/// 字句解析エラー
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum LexErrorKind {
    InvalidChar(char),
    Eof,
}

// 字句解析エラーアノテーション
type LexError = Annot<LexErrorKind>;

impl LexError {
    fn invalid_char(c: char, loc: Loc) -> Self {
        Self::new(LexErrorKind::InvalidChar(c), loc)
    }
    fn eof(loc: Loc) -> Self {
        Self::new(LexErrorKind::Eof, loc)
    }
}

/// 字句解析器
fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let input = input.as_bytes();
    let mut pos = 0;

    // サブレキサを読んだあとにposを更新する
    macro_rules! lex_a_token {
        ($lexer:expr) => {{
            let (tok, p) = $lexer?;
            tokens.push(tok);
            pos = p;
        }};
    }

    while pos < input.len() {
        match input[pos] {
            b'0'..=b'9' => lex_a_token!(lex_number(input, pos)),
            b'+' => lex_a_token!(lex_plus(input, pos)),
            b'-' => lex_a_token!(lex_minus(input, pos)),
            b'*' => lex_a_token!(lex_asterisk(input, pos)),
            b'/' => lex_a_token!(lex_slash(input, pos)),
            b'(' => lex_a_token!(lex_lparen(input, pos)),
            b')' => lex_a_token!(lex_rparen(input, pos)),
            // 空白の場合
            b' ' | b'\t' | b'\n' => {
                let ((), p) = skip_space(input, pos);
                pos = p;
            }
            // それ以外はエラー
            b => return Err(LexError::invalid_char(b as char, Loc(pos, pos + 1))),
        }
    }
    Ok(tokens)
}

/// posの場所のバイトが期待のものか判定し、期待のものであれば１バイト消費してposを1すすめる
fn consume_byte(input: &[u8], pos: usize, b: u8) -> Result<(u8, usize), LexError> {
    // 1バイト消費したいが入力が終わっているのでエラー
    if input.len() <= pos {
        return Err(LexError::eof(Loc(pos, pos)));
    }

    // 入力が期待のものでないのでエラー
    if input[pos] != b {
        return Err(LexError::invalid_char(
            input[pos] as char,
            Loc(pos, pos + 1),
        ));
    }

    Ok((b, pos + 1))
}

fn lex_plus(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    // match consume_byte(input, start, b'+') {
    //     Ok((_, end)) => Ok((Token::plus(Loc(start, end)), end)),
    //     Err(err) => Err(err),
    // }
    // 上記と同様の記述を以下のResult::map（Reuslt<T, E> -> Result<U, E>）を用いて１行で記述できる
    consume_byte(input, start, b'+').map(|(_, end)| (Token::plus(Loc(start, end)), end))
}

fn lex_minus(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'-').map(|(_, end)| (Token::minus(Loc(start, end)), end))
}

fn lex_asterisk(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'*').map(|(_, end)| (Token::asterisk(Loc(start, end)), end))
}

fn lex_slash(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'/').map(|(_, end)| (Token::slash(Loc(start, end)), end))
}

fn lex_lparen(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b'(').map(|(_, end)| (Token::lparen(Loc(start, end)), end))
}

fn lex_rparen(input: &[u8], start: usize) -> Result<(Token, usize), LexError> {
    consume_byte(input, start, b')').map(|(_, end)| (Token::rparen(Loc(start, end)), end))
}

fn lex_number(input: &[u8], pos: usize) -> Result<(Token, usize), LexError> {
    use std::str::from_utf8;
    let start = pos;
    let end = recognize_many(input, start, |b| b"0123456789".contains(&b));

    // 数字の文字列を数値に変換する
    let n = from_utf8(&input[start..end])
        .unwrap() // start..endまでの間は数字が続くのでunwrapしても安全
        .parse()
        .unwrap(); // from_utf8は常に成功するためunwrapしても安全
    Ok((Token::number(n, Loc(start, end)), end))
}

/// 入力に空白文字が続く限りposをすすめる
fn skip_space(input: &[u8], pos: usize) -> ((), usize) {
    let pos = recognize_many(input, pos, |b| b" \n\t".contains(&b));
    ((), pos)
}

/// 条件に当てはまる間、posをすすめる
fn recognize_many(input: &[u8], mut pos: usize, mut f: impl FnMut(u8) -> bool) -> usize {
    while pos < input.len() && f(input[pos]) {
        pos += 1;
    }
    pos
}

/// ASTを表すデータ型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AstKind {
    /// 数値
    Num(u64),
    /// 単項演算
    UniOp { op: UniOp, e: Box<Ast> },
    /// 二項演算
    BinOp { op: BinOp, l: Box<Ast>, r: Box<Ast> },
}

type Ast = Annot<AstKind>;

impl Ast {
    fn num(n: u64, loc: Loc) -> Self {
        Self::new(AstKind::Num(n), loc)
    }
    fn uniop(op: UniOp, e: Ast, loc: Loc) -> Self {
        Self::new(AstKind::UniOp { op, e: Box::new(e) }, loc)
    }
    fn binop(op: BinOp, l: Ast, r: Ast, loc: Loc) -> Self {
        Self::new(
            AstKind::BinOp {
                op,
                l: Box::new(l),
                r: Box::new(r),
            },
            loc,
        )
    }
}

/// 単項演算子を表すデータ型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum UniOpKind {
    /// 正号
    Plus,
    /// 負号
    Minus,
}

type UniOp = Annot<UniOpKind>;

impl UniOp {
    fn plus(loc: Loc) -> Self {
        Self::new(UniOpKind::Plus, loc)
    }
    fn minus(loc: Loc) -> Self {
        Self::new(UniOpKind::Minus, loc)
    }
}

/// 二項演算子を表すデータ型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum BinOpKind {
    /// 加算
    Add,
    /// 減算
    Sub,
    /// 乗算
    Mult,
    /// 除算
    Div,
}

type BinOp = Annot<BinOpKind>;

impl BinOp {
    fn add(loc: Loc) -> Self {
        Self::new(BinOpKind::Add, loc)
    }
    fn sub(loc: Loc) -> Self {
        Self::new(BinOpKind::Sub, loc)
    }
    fn mult(loc: Loc) -> Self {
        Self::new(BinOpKind::Mult, loc)
    }
    fn div(loc: Loc) -> Self {
        Self::new(BinOpKind::Div, loc)
    }
}

/// 構文解析エラー
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ParseError {
    /// 予期しないトークン
    UnexpectedToken(Token),
    /// 式を期待したが式でないものが来た
    NotExpression(Token),
    /// 演算子を期待したが演算子でないものが来た
    NotOperator(Token),
    /// カッコが閉じられていない
    UnclosedOpenParen(Token),
    /// 式の解析が終わったのにまだトークンが残っている
    RedundantExpression(Token),
    /// パース途中で入力が終了した
    Eof,
}

#[test]
fn test_lexer() {
    assert_eq!(
        lex("1 + 2 * 3 - -10"),
        Ok(vec![
            Token::number(1, Loc(0, 1)),
            Token::plus(Loc(2, 3)),
            Token::number(2, Loc(4, 5)),
            Token::asterisk(Loc(6, 7)),
            Token::number(3, Loc(8, 9)),
            Token::minus(Loc(10, 11)),
            Token::minus(Loc(12, 13)),
            Token::number(10, Loc(13, 15)),
        ])
    )
}

fn parse(tokens: Vec<Token>) -> Result<Ast, ParseError> {
    let mut tokens = tokens.into_iter().peekable();
    let ret = parse_expr(&mut tokens)?;
    match tokens.next() {
        Some(tok) => Err(ParseError::RedundantExpression(tok)),
        None => Ok(ret),
    }
}

use std::iter::Peekable;
fn parse_expr<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    parse_expr3(tokens)
}

fn parse_expr3<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    // parse_left_binopにわたす関数を定義する
    fn parse_expr3_op<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<BinOp, ParseError>
    where
        Tokens: Iterator<Item = Token>,
    {
        let op = tokens
            .peek()
            // イテレータの終わりは入力の終端なのでEofを返す
            .ok_or(ParseError::Eof)
            // エラーを返すかもしれない値をつなげる
            .and_then(|tok| match tok.value {
                TokenKind::Plus => Ok(BinOp::add(tok.loc.clone())),
                TokenKind::Minus => Ok(BinOp::sub(tok.loc.clone())),
                _ => Err(ParseError::NotOperator(tok.clone())),
            })?;
        tokens.next();
        Ok(op)
    }
    parse_left_binop(tokens, parse_expr2, parse_expr3_op)
}

fn parse_expr2<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    // `parse_left_binop` に渡す関数を定義する
    fn parse_expr2_op<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<BinOp, ParseError>
    where
        Tokens: Iterator<Item = Token>,
    {
        let op = tokens
            .peek()
            .ok_or(ParseError::Eof)
            .and_then(|tok| match tok.value {
                TokenKind::Asterisk => Ok(BinOp::mult(tok.loc.clone())),
                TokenKind::Slash => Ok(BinOp::div(tok.loc.clone())),
                _ => Err(ParseError::NotOperator(tok.clone())),
            })?;
        tokens.next();
        Ok(op)
    }

    parse_left_binop(tokens, parse_expr1, parse_expr2_op)
}

fn parse_left_binop<Tokens>(
    tokens: &mut Peekable<Tokens>,
    subexpr_parser: fn(&mut Peekable<Tokens>) -> Result<Ast, ParseError>,
    op_parser: fn(&mut Peekable<Tokens>) -> Result<BinOp, ParseError>,
) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    let mut e = subexpr_parser(tokens)?;
    loop {
        match tokens.peek() {
            Some(_) => {
                let op = match op_parser(tokens) {
                    Ok(op) => op,
                    // ここでパースに失敗するとこれ以上中間演算子がないという意味
                    Err(_) => break,
                };
                let r = subexpr_parser(tokens)?;
                let loc = e.loc.merge(&r.loc);
                e = Ast::binop(op, e, r, loc);
            }
            _ => break,
        }
    }
    Ok(e)
}

fn parse_expr1<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    match tokens.peek().map(|tok| tok.value) {
        Some(TokenKind::Plus) | Some(TokenKind::Minus) => {
            // ("+" | "-")
            let op = match tokens.next() {
                Some(Token {
                    value: TokenKind::Plus,
                    loc,
                }) => UniOp::plus(loc),
                Some(Token {
                    value: TokenKind::Minus,
                    loc,
                }) => UniOp::minus(loc),
                _ => unreachable!(),
            };
            // , ATOM
            let e = parse_atom(tokens)?;
            let loc = op.loc.merge(&e.loc);
            Ok(Ast::uniop(op, e, loc))
        }
        // | ATOM
        _ => parse_atom(tokens),
    }
}

fn parse_atom<Tokens>(tokens: &mut Peekable<Tokens>) -> Result<Ast, ParseError>
where
    Tokens: Iterator<Item = Token>,
{
    tokens
        .next()
        .ok_or(ParseError::Eof)
        .and_then(|tok| match tok.value {
            // UNUMBER
            TokenKind::Number(n) => Ok(Ast::new(AstKind::Num(n), tok.loc)),
            // | "(", EXPR3, ")" ;
            TokenKind::LParen => {
                let e = parse_expr(tokens)?;
                match tokens.next() {
                    Some(Token {
                        value: TokenKind::RParen,
                        ..
                    }) => Ok(e),
                    Some(t) => Err(ParseError::RedundantExpression(t)),
                    _ => Err(ParseError::UnclosedOpenParen(tok)),
                }
            }
            _ => Err(ParseError::NotExpression(tok)),
        })
}

#[test]
fn test_parser() {
    // 1 + 2 * 3 - -10
    let ast = parse(vec![
        Token::number(1, Loc(0, 1)),
        Token::plus(Loc(2, 3)),
        Token::number(2, Loc(4, 5)),
        Token::asterisk(Loc(6, 7)),
        Token::number(3, Loc(8, 9)),
        Token::minus(Loc(10, 11)),
        Token::minus(Loc(12, 13)),
        Token::number(10, Loc(13, 15)),
    ]);
    assert_eq!(
        ast,
        Ok(Ast::binop(
            BinOp::sub(Loc(10, 11)),
            Ast::binop(
                BinOp::add(Loc(2, 3)),
                Ast::num(1, Loc(0, 1)),
                Ast::binop(
                    BinOp::new(BinOpKind::Mult, Loc(6, 7)),
                    Ast::num(2, Loc(4, 5)),
                    Ast::num(3, Loc(8, 9)),
                    Loc(4, 9)
                ),
                Loc(0, 9),
            ),
            Ast::uniop(
                UniOp::minus(Loc(12, 13)),
                Ast::num(10, Loc(13, 15)),
                Loc(12, 15)
            ),
            Loc(0, 15)
        ))
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Error {
    Lexer(LexError),
    Parser(ParseError),
}

impl From<LexError> for Error {
    fn from(e: LexError) -> Self {
        Error::Lexer(e)
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::Parser(e)
    }
}

use std::str::FromStr;
impl FromStr for Ast {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = lex(s)?;
        let ast = parse(tokens)?;
        Ok(ast)
    }
}

use std::fmt;
impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::TokenKind::*;
        match self {
            Number(n) => n.fmt(f),
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            Asterisk => write!(f, "*"),
            Slash => write!(f, "/"),
            LParen => write!(f, "("),
            RParen => write!(f, ")"),
        }
    }
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.0, self.1)
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::LexErrorKind::*;
        let loc = &self.loc;
        match self.value {
            InvalidChar(c) => write!(f, "{}: invalid char '{}'", loc, c),
            Eof => write!(f, "End of file"),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ParseError::*;
        match self {
            UnexpectedToken(tok) => write!(f, "{}: '{}' is not expected", tok.loc, tok.value),
            NotExpression(tok) => write!(
                f,
                "{}: '{}' is not a start of expression",
                tok.loc, tok.value
            ),
            NotOperator(tok) => write!(f, "{}: '{}' is not an operator", tok.loc, tok.value),
            UnclosedOpenParen(tok) => write!(f, "{}: '{}' is not closed", tok.loc, tok.value),
            RedundantExpression(tok) => write!(
                f,
                "{}: expression after '{}' is redundant",
                tok.loc, tok.value
            ),
            Eof => write!(f, "End of file"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "parser error")
    }
}

use std::error::Error as StdError;
impl StdError for LexError {}
impl StdError for ParseError {}
impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        use self::Error::*;
        match self {
            Lexer(lex) => Some(lex),
            Parser(parse) => Some(parse),
        }
    }
}

fn print_annot(input: &str, loc: Loc) {
    eprintln!("{}", input);
    eprintln!("{}{}", " ".repeat(loc.0), "^".repeat(loc.1 - loc.0));
}

impl Error {
    fn show_diagnostic(&self, input: &str) {
        use self::Error::*;
        use self::ParseError as P;
        let (e, loc): (&dyn StdError, Loc) = match self {
            Lexer(e) => (e, e.loc.clone()),
            Parser(e) => {
                let loc = match e {
                    P::UnexpectedToken(Token { loc, .. })
                    | P::NotExpression(Token { loc, .. })
                    | P::NotOperator(Token { loc, .. })
                    | P::UnclosedOpenParen(Token { loc, .. }) => loc.clone(),
                    P::RedundantExpression(Token { loc, .. }) => Loc(loc.0, input.len()),
                    P::Eof => Loc(input.len(), input.len() + 1),
                };
                (e, loc)
            }
        };
        eprintln!("{}", e);
        print_annot(input, loc);
    }
}

fn show_trace(e: impl StdError) {
    // エラーの場合、エラーとsourceをすべて出力する
    eprintln!("{}", e);
    let mut source = e.source();
    // sourceをすべてたどって表示する
    while let Some(e) = source {
        eprintln!("caused by {}", e);
        source = e.source();
    }
}

/// インタプリタを表すデータ型
struct InterPreter;

impl InterPreter {
    pub fn new() -> Self {
        InterPreter
    }
    pub fn eval(&mut self, expr: &Ast) -> Result<i64, InterpreterError> {
        use self::AstKind::*;
        match expr.value {
            Num(n) => Ok(n as i64),
            UniOp { ref op, ref e } => {
                let e = self.eval(e)?;
                Ok(self.eval_uniop(op, e))
            }
            BinOp {
                ref op,
                ref l,
                ref r,
            } => {
                let l = self.eval(l)?;
                let r = self.eval(r)?;
                self.eval_binop(op, l, r)
                    .map_err(|e| InterpreterError::new(e, expr.loc.clone()))
            }
        }
    }

    fn eval_uniop(&mut self, op: &UniOp, n: i64) -> i64 {
        use self::UniOpKind::*;
        match op.value {
            Plus => n,
            Minus => -n,
        }
    }

    fn eval_binop(&mut self, op: &BinOp, l: i64, r: i64) -> Result<i64, InterpreterErrorKind> {
        use self::BinOpKind::*;
        match op.value {
            Add => Ok(l + r),
            Sub => Ok(l - r),
            Mult => Ok(l * r),
            Div => {
                if r == 0 {
                    Err(InterpreterErrorKind::DvisionByZero)
                } else {
                    Ok(l / r)
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum InterpreterErrorKind {
    DvisionByZero,
}

type InterpreterError = Annot<InterpreterErrorKind>;

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::InterpreterErrorKind::*;
        match self.value {
            DvisionByZero => write!(f, "division by zero"),
        }
    }
}

impl StdError for InterpreterError {
    fn description(&self) -> &str {
        use self::InterpreterErrorKind::*;
        match self.value {
            DvisionByZero => "the right hand expression of the division evaluates to zero",
        }
    }
}

impl InterpreterError {
    fn show_diagnostic(&self, input: &str) {
        eprintln!("{}", self);
        print_annot(input, self.loc.clone());
    }
}

use std::io;
fn prompt(s: &str) -> io::Result<()> {
    use std::io::{stdout, Write};
    let stdout = stdout();
    let mut stdout = stdout.lock();
    stdout.write(s.as_bytes())?;
    stdout.flush()
}

fn main() {
    use std::io::{stdin, BufRead, BufReader};
    let stdin = stdin();
    let stdin = stdin.lock();
    let stdin = BufReader::new(stdin);
    let mut lines = stdin.lines();

    loop {
        prompt("> ").unwrap();
        // ユーザーの入力受付
        if let Some(Ok(line)) = lines.next() {
            let ast = match line.parse::<Ast>() {
                Ok(ast) => ast,
                Err(e) => {
                    e.show_diagnostic(&line);
                    show_trace(e);
                    // エラー表示のあとは次の入力を受け付ける
                    continue;
                }
            };
            println!("{:?}", ast);
        } else {
            break;
        }
    }
}
