/// This file is maintained by rustemo but can be modified manually.
/// All manual changes will be preserved except non-doc comments.
use rustemo::Token as RustemoToken;
use super::calculator::{TokenKind, Context};
pub type Input = str;
pub type Ctx<'i> = Context<'i, Input>;
#[allow(dead_code)]
pub type Token<'i> = RustemoToken<'i, Input, TokenKind>;
pub type Number = String;
pub fn number(_ctx: &Ctx, token: Token) -> Number {
    token.value.into()
}
#[derive(Debug, Clone)]
pub struct Add {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub struct Sub {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub struct Mul {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub struct Div {
    pub e_1: Box<E>,
    pub e_3: Box<E>,
}
#[derive(Debug, Clone)]
pub enum E {
    Add(Add),
    Sub(Sub),
    Mul(Mul),
    Div(Div),
    Number(Number),
}
pub fn e_add(_ctx: &Ctx, e_1: E, e_3: E) -> E {
    E::Add(Add {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_sub(_ctx: &Ctx, e_1: E, e_3: E) -> E {
    E::Sub(Sub {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_mul(_ctx: &Ctx, e_1: E, e_3: E) -> E {
    E::Mul(Mul {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_div(_ctx: &Ctx, e_1: E, e_3: E) -> E {
    E::Div(Div {
        e_1: Box::new(e_1),
        e_3: Box::new(e_3),
    })
}
pub fn e_number(_ctx: &Ctx, number: Number) -> E {
    E::Number(number)
}
#[derive(Debug, Clone)]
pub struct ArrayNoO {
    pub e: E,
    pub array: Box<Array>,
}
pub type Array = Option<ArrayNoO>;
pub fn array_c1(_ctx: &Ctx, e: E, array: Array) -> Array {
    Some(ArrayNoO {
        e,
        array: Box::new(array),
    })
}
pub fn array_empty(_ctx: &Ctx) -> Array {
    None
}
