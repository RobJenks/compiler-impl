use std::slice::Iter;
use core::iter::Peekable;
use crate::tokens::token::Token;

pub mod token;

pub type TokenIter<'a> = Peekable<Iter<'a, token::Token>>;

pub struct Tokenized {
    data : Vec<Token>
}

impl Tokenized {
    pub fn new() -> Self {
        Self { data : vec![] }
    }

    pub fn add(&mut self, token: Token) {
        self.data.push(token);
    }

    pub fn data(&self) -> &Vec<Token> {
        &self.data
    }

    pub fn filter(&mut self, pred: fn(&Token) -> bool) {
        self.data = self.data
            .iter()
            .filter(|&t| pred(t))
            .map(|t| t.clone())
            .collect();
    }
}


