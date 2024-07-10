use crate::error::Error;

use std::iter::Enumerate;
use std::ops::{Range, RangeFrom, RangeFull, RangeTo};

use logos::{Lexer, Logos};
use nom::{InputIter, InputLength, InputTake, Needed, Slice};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'a> {
    // Source kml
    pub source: &'a str,
    pub kind: TokenKind,
    // Left closed, right open
    pub span: Range<usize>,
}

impl<'a> Token<'a> {
    pub fn text(&self) -> &'a str {
        &self.source[self.span.clone()]
    }
}

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token({:?}, {})", self.kind, self.text())
    }
}

#[derive(Debug, Clone)]
pub struct Tokens<'a> {
    pub tok: &'a [Token<'a>],
    pub start: usize,
    pub end: usize,
}

impl<'a> From<&'a [Token<'a>]> for Tokens<'a> {
    fn from(tok: &'a [Token<'a>]) -> Self {
        Tokens {
            tok,
            start: 0,
            end: tok.len(),
        }
    }
}

impl<'a> InputLength for Tokens<'a> {
    #[inline]
    fn input_len(&self) -> usize {
        self.tok.len()
    }
}

impl<'a> InputTake for Tokens<'a> {
    #[inline]
    fn take(&self, count: usize) -> Self {
        Tokens {
            tok: &self.tok[0..count],
            start: 0,
            end: count,
        }
    }

    #[inline]
    fn take_split(&self, count: usize) -> (Self, Self) {
        let (prefix, suffix) = self.tok.split_at(count);
        let first = Tokens {
            tok: prefix,
            start: 0,
            end: prefix.len(),
        };
        let second = Tokens {
            tok: suffix,
            start: 0,
            end: suffix.len(),
        };
        (second, first)
    }
}

impl<'a> InputLength for Token<'a> {
    #[inline]
    fn input_len(&self) -> usize {
        1
    }
}

impl<'a> Slice<Range<usize>> for Tokens<'a> {
    #[inline]
    fn slice(&self, range: Range<usize>) -> Self {
        Tokens {
            tok: self.tok.slice(range.clone()),
            start: self.start + range.start,
            end: self.start + range.end,
        }
    }
}

impl<'a> Slice<RangeTo<usize>> for Tokens<'a> {
    #[inline]
    fn slice(&self, range: RangeTo<usize>) -> Self {
        self.slice(0..range.end)
    }
}

impl<'a> Slice<RangeFrom<usize>> for Tokens<'a> {
    #[inline]
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        self.slice(range.start..self.end - self.start)
    }
}

impl<'a> Slice<RangeFull> for Tokens<'a> {
    #[inline]
    fn slice(&self, _: RangeFull) -> Self {
        Tokens {
            tok: self.tok,
            start: self.start,
            end: self.end,
        }
    }
}

impl<'a> InputIter for Tokens<'a> {
    type Item = &'a Token<'a>;
    type Iter = Enumerate<::std::slice::Iter<'a, Token<'a>>>;
    type IterElem = ::std::slice::Iter<'a, Token<'a>>;

    #[inline]
    fn iter_indices(&self) -> Enumerate<::std::slice::Iter<'a, Token<'a>>> {
        self.tok.iter().enumerate()
    }
    #[inline]
    fn iter_elements(&self) -> ::std::slice::Iter<'a, Token<'a>> {
        self.tok.iter()
    }
    #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.tok.iter().position(predicate)
    }
    #[inline]
    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        if self.tok.len() >= count {
            Ok(count)
        } else {
            Err(Needed::Unknown)
        }
    }
}

pub struct Tokenizer<'a> {
    source: &'a str,
    lexer: Lexer<'a, TokenKind>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Tokenizer {
            source,
            lexer: TokenKind::lexer(source),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lexer.next() {
            Some(kind) => match kind {
                Ok(kind) => Some(Ok(Token {
                    source: self.source,
                    kind,
                    span: self.lexer.span(),
                })),
                Err(_) => Some(Err(Error::LexError)),
            },
            None => None,
        }
    }
}

#[derive(Logos, Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
    // Skip

    #[regex(r"[ \t\r\n\f]+", logos::skip)]
    Whitespace,

    #[regex(r"#[^\n\f]*", logos::skip)]
    Comment,

    // Identifier

    #[regex(r#"[_a-zA-Z][_a-zA-Z0-9]*"#)]
    Identifier,
    
    // Event defs

    #[token("event", ignore(ascii_case))]
    Event,

    // Built-in actions

    #[token("sched", ignore(ascii_case))]
    Sched,

    #[token("newtask", ignore(ascii_case))]
    NewTask,

    #[token("exit", ignore(ascii_case))]
    Exit,

    #[token("shutdown", ignore(ascii_case))]
    Shutdown,

    #[token("stop", ignore(ascii_case))]
    Stop,

    // Kernel configs

    #[token("kernel", ignore(ascii_case))]
    Kernel,

    #[token("events", ignore(ascii_case))]
    Events,

    #[token("scheduler", ignore(ascii_case))]
    Scheduler,

    // Scheduler types

    #[token("fifo", ignore(ascii_case))]
    Fifo,

    #[token("random", ignore(ascii_case))]
    Random,

    // Other markers

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token(",")]
    Comma,

    #[token("=")]
    Eq,
}

impl TokenKind {
    pub fn is_action(&self) -> bool {
        match *self {
            Self::Sched | Self::Stop | Self::Shutdown | Self::Exit | Self::NewTask => true,
            _ => false,
        }
    }
}

pub fn tokenize_kml(kml: &str) -> Result<Vec<Token>, Error> {
    Tokenizer::new(kml).collect::<Result<Vec<_>, _>>()
}
