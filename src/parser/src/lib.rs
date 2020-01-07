use common::tokens::token::Token;

pub struct Parser {
}

impl Parser {
    pub fn new() -> Self { Self {} }

    pub fn parse(_tokens: Vec<Token>) -> () {

    }

}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
