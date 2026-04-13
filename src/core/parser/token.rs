pub enum Token {
    Number { raw: String, base: u8 },
    Variable(String),
    Operation { sign: String, operands: Vec<Box<Token>> }
}