use crate::tokens::TokenIter;

pub struct NodeParseResult<'a,T>
    where T: Clone {

    result: Option<T>,
    next: Option<TokenIter<'a>>
}

impl <'a,T> NodeParseResult<'a,T>
    where T: Clone,T: std::fmt::Debug {

    pub fn of(res: T, next: TokenIter<'a>) -> Self {
        Self { result: Some(res), next: Some(next) }
    }

    pub fn of_opt(res: T, next: Option<TokenIter<'a>>) -> Self {
        Self { result: Some(res), next }
    }

    pub fn and<'b,U>(&self, f: fn(TokenIter<'a>) -> NodeParseResult<'b,U>) -> NodeParseResult<'b,(T,U)>
        where U: Clone {
        match &self.result {
            None => NodeParseResult { result: None, next: None },
            _ => match &self.next {
                None => panic!("Invalid state; cannot compose without source iterator"),
                Some(it) => {
                    let rhs = f(it.clone());
                    match rhs.result {
                        None => NodeParseResult { result: None, next: None },
                        Some(next_res) => NodeParseResult {
                            result: self.result.clone().map(|x| (x, next_res)),
                            next: rhs.next
                        }
                    }
                }
            }
        }
    }

    pub fn result(&self) -> &Option<T> {
        &self.result
    }

    pub fn next(&'a self) -> &Option<TokenIter<'a>> {
        &self.next
    }

    pub fn output(&self) -> (Option<T>, Option<TokenIter<'a>>) {
        (self.result.clone(), self.next.clone())
    }


    pub fn none() -> Self {
        Self { result: None, next: None }
    }

    pub fn is_valid(&self) -> bool {
        self.result.is_some()
    }

}



//use crate::tokens::TokenIter;
//
//pub type NodeParseResult<'a,T> = Option<(T, TokenIter<'a>)>;
//
//pub trait ComposableParseResult<'a,T,U> {
//    fn and(x: NodeParseResult<'a,T>) -> NodeParseResult<'a,(T,U)>;
//}
//
//impl <'a,T,U> ComposableParseResult<'a,T,U> for NodeParseResult<'a,T> {
//    fn and(x: NodeParseResult<'a,T>) -> NodeParseResult<'a,(T,U)> {
//
//    }
//}