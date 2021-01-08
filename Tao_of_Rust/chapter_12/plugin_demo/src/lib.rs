#![feature(plugin_registrar, rustc_private)]
extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;
use self::syntax::parse::token;
use self::syntax::tokenstream::TokenStream;
use self::syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use self::syntax::ext::build::AstBuilder;
use self::syntax::ext::quote::rt::Span;
use self::rustc_plugin::Registry;

static ROMAN_NUMERALS: &'static [(&'static str, usize)] = &[
    ("M", 1000), ("CM", 900), ("D", 500), ("CD", 400),
    ("C", 100), ("XM", 90), ("L", 50), ("XL", 40),
    ("X", 10), ("IX", 9), ("V", 5), ("IV", 4),
    ("I", 1)
];

fn expand_roman(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree])
    -> Box<MacResult + 'static>{
    let text = match args[0] {
        TokenTree::Token(_, token::Ident(s, _)) => s.to_string(),
        _ => {
            cx.span_err(sp, "argument should be a single identifier");
            return DummyResult::any(sp);
        }
    };
    let mut test = &*test;
    let mut total = 0;
    while !text.is_empty() {
        match ROMAN_NUMERALS
            .iter().find(|&&(rn, _)| test.starts_with(rn))
        {
            Some(&(rn, val)) => {
                total += val;
                text = &text[rn.len()..];
            }
            None => {
                cx.span_err(sp, "invalid Roman numeral");
                return DummyResult::any(sp);
            }
        }
    }
    MacResult::expr(cx.expr_usize(sp, total))
}

#[plugin_registrar]
pub fn roman_to_digit(reg: &mut Registry) {
    reg.register_macro("roman_to_digit", expand_roman);
}