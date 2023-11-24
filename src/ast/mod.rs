use alloc::{boxed::Box, string::String, vec::Vec};

#[derive(Clone, Debug)]
pub struct Module {
    pub statements: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Ident(pub String);

impl From<&str> for Ident {
    fn from(s: &str) -> Self {
        Self(String::from(s))
    }
}

impl From<String> for Ident {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl Ident {
    pub fn matches(&self, s: &str) -> bool {
        self.0 == s
    }
}

#[derive(Clone, Debug)]
pub enum Statement {
    Function(Span, Ident, Vec<Ident>, Vec<Statement>),
    Expr(Expr),
}

pub type Span = core::ops::Range<usize>;

#[derive(Clone, Debug)]
pub enum Expr {
    Literal(Span, Literal),
    List(Span, Vec<Expr>),
    Let(Ident, Box<Expr>, Box<Expr>),
    Then(Box<Expr>, Box<Expr>),
    Ident(Span, Ident),
    Call(Span, Vec<Expr>),
    If {
        span: Span,
        cond: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
    },
}

#[derive(Clone, Debug)]
pub enum Literal {
    String(String),
    Number(Number),
    Decimal(Decimal),
    Bytes(Box<[u8]>),
}

#[cfg(feature = "backend-bignum")]
use num_traits::Num;

#[cfg(feature = "backend-bignum")]
use core::str::FromStr;

#[cfg(feature = "backend-bignum")]
pub type NumberInner = num_bigint::BigInt;

#[cfg(feature = "backend-smallnum")]
pub type NumberInner = u64;

#[derive(Clone, Debug)]
pub struct Number(pub NumberInner);

impl Number {
    pub fn from_str_radix(s: &str, n: u32) -> Result<Self, ()> {
        NumberInner::from_str_radix(s, n)
            .map(|n| Self(n))
            .map_err(|_| ())
    }
}

#[cfg(feature = "backend-bignum")]
pub type DecimalInner = bigdecimal::BigDecimal;

#[cfg(feature = "backend-smallnum")]
pub type DecimalInner = f64;

#[derive(Clone, Debug)]
pub struct Decimal(pub DecimalInner);

impl Decimal {
    pub fn from_str(s: &str) -> Result<Self, ()> {
        #[cfg(feature = "backend-bignum")]
        {
            DecimalInner::from_str(s).map(|n| Self(n)).map_err(|_| ())
        }
        #[cfg(feature = "backend-smallnum")]
        {
            use core::str::FromStr;
            DecimalInner::from_str(s).map(|n| Self(n)).map_err(|_| ())
        }
    }
}
