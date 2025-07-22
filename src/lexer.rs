//! Lexical analysis for the strongly typed TypeScript compiler
//!
//! This module provides tokenization of TypeScript source code into a stream
//! of tokens that can be consumed by the parser.

use crate::error::{DrafError, DrafResult};
use logos::Logos;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a token in the TypeScript source code
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
    pub start: usize,
    pub end: usize,
}

impl Token {
    /// Create a new token
    pub fn new(
        kind: TokenKind,
        lexeme: String,
        line: usize,
        column: usize,
        start: usize,
        end: usize,
    ) -> Self {
        Self {
            kind,
            lexeme,
            line,
            column,
            start,
            end,
        }
    }

    /// Check if this token is a keyword
    pub fn is_keyword(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::Let
                | TokenKind::Const
                | TokenKind::Var
                | TokenKind::Function
                | TokenKind::If
                | TokenKind::Else
                | TokenKind::For
                | TokenKind::While
                | TokenKind::Break
                | TokenKind::Continue
                | TokenKind::Return
                | TokenKind::True
                | TokenKind::False
                | TokenKind::Null
                | TokenKind::Undefined
                | TokenKind::Any
                | TokenKind::Number
                | TokenKind::String
                | TokenKind::Boolean
                | TokenKind::Void
                | TokenKind::Never
                | TokenKind::Interface
                | TokenKind::Class
                | TokenKind::Type
                | TokenKind::Export
                | TokenKind::Import
        )
    }

    /// Check if this token is an operator
    pub fn is_operator(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::Plus
                | TokenKind::Minus
                | TokenKind::Star
                | TokenKind::Slash
                | TokenKind::Percent
                | TokenKind::Equal
                | TokenKind::EqualEqual
                | TokenKind::NotEqual
                | TokenKind::Less
                | TokenKind::LessEqual
                | TokenKind::Greater
                | TokenKind::GreaterEqual
                | TokenKind::AndAnd
                | TokenKind::OrOr
                | TokenKind::Bang
        )
    }

    /// Check if this token is a literal
    pub fn is_literal(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::NumberLiteral
                | TokenKind::StringLiteralDouble
                | TokenKind::StringLiteralSingle
                | TokenKind::TemplateLiteral
                | TokenKind::True
                | TokenKind::False
                | TokenKind::Null
                | TokenKind::Undefined
        )
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}({})", self.kind, self.lexeme)
    }
}

/// Token types recognized by the lexer
#[derive(Logos, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TokenKind {
    // Literals
    #[regex(r"[0-9]+(\.[0-9]+)?([eE][+-]?[0-9]+)?")]
    NumberLiteral,

    #[regex(r#""([^"\\]|\\.)*""#)]
    StringLiteralDouble,

    #[regex(r#"'([^'\\]|\\.)*'"#)]
    StringLiteralSingle,

    #[regex(r"`([^`\\]|\\.)*`")]
    TemplateLiteral,

    // Keywords
    #[token("let")]
    Let,
    #[token("const")]
    Const,
    #[token("var")]
    Var,
    #[token("function")]
    Function,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("for")]
    For,
    #[token("while")]
    While,
    #[token("return")]
    Return,
    #[token("break")]
    Break,
    #[token("continue")]
    Continue,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("null")]
    Null,
    #[token("undefined")]
    Undefined,

    // Type keywords
    #[token("any")]
    Any,
    #[token("number")]
    Number,
    #[token("string")]
    String,
    #[token("boolean")]
    Boolean,
    #[token("void")]
    Void,
    #[token("never")]
    Never,
    #[token("interface")]
    Interface,
    #[token("class")]
    Class,
    #[token("type")]
    Type,
    #[token("export")]
    Export,
    #[token("import")]
    Import,
    #[token("console")]
    Console,

    // Identifiers
    #[regex(r"[a-zA-Z_$][a-zA-Z0-9_$]*")]
    Identifier,

    // Operators
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("=")]
    Equal,
    #[token("===")]
    StrictEqual,
    #[token("!==")]
    StrictNotEqual,
    #[token("==")]
    EqualEqual,
    #[token("!=")]
    NotEqual,
    #[token("<")]
    Less,
    #[token("<=")]
    LessEqual,
    #[token(">")]
    Greater,
    #[token(">=")]
    GreaterEqual,
    #[token("&&")]
    AndAnd,
    #[token("||")]
    OrOr,
    #[token("??")]
    NullishCoalescing,
    #[token("!")]
    Bang,

    // Punctuation
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token(",")]
    Comma,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token(".")]
    Dot,
    #[token("?")]
    Question,
    #[token("=>")]
    Arrow,
    #[token("|")]
    Pipe,
    #[token("&")]
    Ampersand,

    // Whitespace and comments (ignored)
    #[regex(r"[ \t\f]+", logos::skip)]
    #[regex(r"//[^\n]*", logos::skip)]
    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    Whitespace,

    // Newlines (significant for ASI - Automatic Semicolon Insertion)
    #[token("\n")]
    #[token("\r\n")]
    Newline,

    // End of file
    Eof,

    // Error token for invalid input
    Error,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            TokenKind::NumberLiteral => "number literal",
            TokenKind::StringLiteralDouble => "string literal (double quoted)",
            TokenKind::StringLiteralSingle => "string literal (single quoted)",
            TokenKind::TemplateLiteral => "template literal",
            TokenKind::Let => "'let'",
            TokenKind::Const => "'const'",
            TokenKind::Var => "'var'",
            TokenKind::Function => "'function'",
            TokenKind::If => "'if'",
            TokenKind::Else => "'else'",
            TokenKind::For => "'for'",
            TokenKind::While => "'while'",
            TokenKind::Return => "'return'",
            TokenKind::Break => "'break'",
            TokenKind::Continue => "'continue'",
            TokenKind::True => "'true'",
            TokenKind::False => "'false'",
            TokenKind::Null => "'null'",
            TokenKind::Undefined => "'undefined'",
            TokenKind::Any => "'any'",
            TokenKind::Number => "'number'",
            TokenKind::String => "'string'",
            TokenKind::Boolean => "'boolean'",
            TokenKind::Void => "'void'",
            TokenKind::Never => "'never'",
            TokenKind::Interface => "'interface'",
            TokenKind::Class => "'class'",
            TokenKind::Type => "'type'",
            TokenKind::Export => "'export'",
            TokenKind::Import => "'import'",
            TokenKind::Console => "'console'",
            TokenKind::Identifier => "identifier",
            TokenKind::Plus => "'+'",
            TokenKind::Minus => "'-'",
            TokenKind::Star => "'*'",
            TokenKind::Slash => "'/'",
            TokenKind::Percent => "'%'",
            TokenKind::Equal => "'='",
            TokenKind::StrictEqual => "'==='",
            TokenKind::StrictNotEqual => "'!=='",
            TokenKind::EqualEqual => "'=='",
            TokenKind::NotEqual => "'!='",
            TokenKind::Less => "'<'",
            TokenKind::LessEqual => "'<='",
            TokenKind::Greater => "'>'",
            TokenKind::GreaterEqual => "'>='",
            TokenKind::AndAnd => "'&&'",
            TokenKind::OrOr => "'||'",
            TokenKind::NullishCoalescing => "'??'",
            TokenKind::Bang => "'!'",
            TokenKind::LeftParen => "'('",
            TokenKind::RightParen => "')'",
            TokenKind::LeftBrace => "'{'",
            TokenKind::RightBrace => "'}'",
            TokenKind::LeftBracket => "'['",
            TokenKind::RightBracket => "']'",
            TokenKind::Comma => "','",
            TokenKind::Semicolon => "';'",
            TokenKind::Colon => "':'",
            TokenKind::Dot => "'.'",
            TokenKind::Question => "'?'",
            TokenKind::Arrow => "'=>'",
            TokenKind::Pipe => "'|'",
            TokenKind::Ampersand => "'&'",
            TokenKind::Whitespace => "whitespace",
            TokenKind::Newline => "newline",
            TokenKind::Eof => "end of file",
            TokenKind::Error => "invalid token",
        };
        write!(f, "{}", name)
    }
}

/// Position tracking for the lexer
#[derive(Debug, Clone)]
struct Position {
    line: usize,
    column: usize,
    offset: usize,
}

impl Position {
    fn new() -> Self {
        Self {
            line: 1,
            column: 1,
            offset: 0,
        }
    }

    fn advance(&mut self, ch: char) {
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        self.offset += ch.len_utf8();
    }
}

/// Tokenize TypeScript source code into a vector of tokens
pub fn tokenize(source: &str) -> DrafResult<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut lexer = TokenKind::lexer(source);
    let mut position = Position::new();
    let mut last_end = 0;

    while let Some(token_kind) = lexer.next() {
        let span = lexer.span();
        let lexeme = lexer.slice().to_string();

        // Update position for skipped characters
        for ch in source[last_end..span.start].chars() {
            position.advance(ch);
        }

        let token_start_line = position.line;
        let token_start_column = position.column;

        // Update position for current token
        for ch in lexeme.chars() {
            position.advance(ch);
        }

        match token_kind {
            Ok(TokenKind::Error) => {
                return Err(DrafError::lexer_error(
                    token_start_line,
                    token_start_column,
                    format!("Unexpected character: '{}'", lexeme),
                ));
            }
            Ok(TokenKind::Whitespace) => {
                // Skip whitespace tokens
                last_end = span.end;
                continue;
            }
            Ok(kind) => {
                let token = Token::new(
                    kind,
                    lexeme,
                    token_start_line,
                    token_start_column,
                    span.start,
                    span.end,
                );
                tokens.push(token);
            }
            Err(_) => {
                return Err(DrafError::lexer_error(
                    token_start_line,
                    token_start_column,
                    format!("Lexer error: '{}'", lexeme),
                ));
            }
        }

        last_end = span.end;
    }

    // Add EOF token
    tokens.push(Token::new(
        TokenKind::Eof,
        String::new(),
        position.line,
        position.column,
        source.len(),
        source.len(),
    ));

    Ok(tokens)
}

/// Check if a string is a reserved keyword
pub fn is_keyword(s: &str) -> bool {
    matches!(
        s,
        "let"
            | "const"
            | "var"
            | "function"
            | "if"
            | "else"
            | "for"
            | "while"
            | "break"
            | "continue"
            | "return"
            | "true"
            | "false"
            | "null"
            | "undefined"
            | "any"
            | "number"
            | "string"
            | "boolean"
            | "void"
            | "never"
            | "interface"
            | "class"
            | "type"
            | "export"
            | "import"
            | "console"
    )
}

/// Get the precedence of a binary operator token
pub fn get_precedence(token: &TokenKind) -> Option<u8> {
    match token {
        TokenKind::OrOr => Some(1),
        TokenKind::NullishCoalescing => Some(2),
        TokenKind::AndAnd => Some(3),
        TokenKind::EqualEqual
        | TokenKind::NotEqual
        | TokenKind::StrictEqual
        | TokenKind::StrictNotEqual => Some(4),
        TokenKind::Less | TokenKind::LessEqual | TokenKind::Greater | TokenKind::GreaterEqual => {
            Some(5)
        }
        TokenKind::Plus | TokenKind::Minus => Some(6),
        TokenKind::Star | TokenKind::Slash | TokenKind::Percent => Some(7),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokenization() {
        let source = "let x: number = 42;";
        let tokens = tokenize(source).unwrap();

        assert_eq!(tokens.len(), 7); // let, x, :, number, =, 42, ;, EOF

        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert_eq!(tokens[1].kind, TokenKind::Identifier);
        assert_eq!(tokens[1].lexeme, "x");
        assert_eq!(tokens[2].kind, TokenKind::Colon);
        assert_eq!(tokens[3].kind, TokenKind::Number);
        assert_eq!(tokens[4].kind, TokenKind::Equal);
        assert_eq!(tokens[5].kind, TokenKind::NumberLiteral);
        assert_eq!(tokens[5].lexeme, "42");
        assert_eq!(tokens[6].kind, TokenKind::Semicolon);
    }

    #[test]
    fn test_string_literals() {
        let source = r#"let name = "hello world";"#;
        let tokens = tokenize(source).unwrap();

        let string_token = tokens
            .iter()
            .find(|t| t.kind == TokenKind::StringLiteral)
            .unwrap();
        assert_eq!(string_token.lexeme, r#""hello world""#);
    }

    #[test]
    fn test_operators() {
        let source = "x + y - z * w / v % u";
        let tokens = tokenize(source).unwrap();

        let operators: Vec<&TokenKind> = tokens
            .iter()
            .filter(|t| t.is_operator())
            .map(|t| &t.kind)
            .collect();

        assert_eq!(
            operators,
            vec![
                &TokenKind::Plus,
                &TokenKind::Minus,
                &TokenKind::Star,
                &TokenKind::Slash,
                &TokenKind::Percent
            ]
        );
    }

    #[test]
    fn test_keywords() {
        let source = "let const var function if else true false null undefined";
        let tokens = tokenize(source).unwrap();

        let keywords: Vec<&TokenKind> = tokens
            .iter()
            .filter(|t| t.is_keyword())
            .map(|t| &t.kind)
            .collect();

        assert_eq!(keywords.len(), 10);
        assert!(keywords.contains(&&TokenKind::Let));
        assert!(keywords.contains(&&TokenKind::Const));
        assert!(keywords.contains(&&TokenKind::True));
        assert!(keywords.contains(&&TokenKind::False));
    }

    #[test]
    fn test_position_tracking() {
        let source = "let x = 1;\nlet y = 2;";
        let tokens = tokenize(source).unwrap();

        // Find the second 'let' token
        let second_let = tokens
            .iter()
            .skip(1)
            .find(|t| t.kind == TokenKind::Let)
            .unwrap();

        assert_eq!(second_let.line, 2);
        assert_eq!(second_let.column, 1);
    }

    #[test]
    fn test_comments_ignored() {
        let source = r#"
            let x = 42; // This is a comment
            /* This is a
               block comment */
            let y = 24;
        "#;
        let tokens = tokenize(source).unwrap();

        // Should not contain any comment tokens
        assert!(!tokens.iter().any(|t| t.lexeme.contains("//")));
        assert!(!tokens.iter().any(|t| t.lexeme.contains("/*")));

        // Should still contain the variable declarations
        let let_tokens: Vec<_> = tokens.iter().filter(|t| t.kind == TokenKind::Let).collect();
        assert_eq!(let_tokens.len(), 2);
    }

    #[test]
    fn test_error_handling() {
        let source = "let x = @invalid;";
        let result = tokenize(source);

        assert!(result.is_err());
        match result.unwrap_err() {
            DrafError::LexerError { line, column, .. } => {
                assert_eq!(line, 1);
                assert!(column > 0);
            }
            _ => panic!("Expected lexer error"),
        }
    }

    #[test]
    fn test_precedence() {
        assert_eq!(get_precedence(&TokenKind::Plus), Some(5));
        assert_eq!(get_precedence(&TokenKind::Star), Some(6));
        assert_eq!(get_precedence(&TokenKind::AndAnd), Some(2));
        assert_eq!(get_precedence(&TokenKind::OrOr), Some(1));
        assert_eq!(get_precedence(&TokenKind::Identifier), None);
    }

    #[test]
    fn test_type_annotations() {
        let source = "let x: number | string | null = getValue();";
        let tokens = tokenize(source).unwrap();

        // Check for type union syntax
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Pipe));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Number));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::String));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Null));
    }

    #[test]
    fn test_function_syntax() {
        let source = "function add(a: number, b: number): number => a + b;";
        let tokens = tokenize(source).unwrap();

        assert!(tokens.iter().any(|t| t.kind == TokenKind::Function));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Arrow));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::LeftParen));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::RightParen));
        assert!(tokens.iter().any(|t| t.kind == TokenKind::Colon));
    }
}
