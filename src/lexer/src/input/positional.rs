// Reference to underlying input data stream and a position within it
pub struct Positional <'a> {
    data: &'a Vec<char>,
    index: usize
}