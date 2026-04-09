pub enum Token {
    Number(NumberBody),
    Variable(String),
    Operation { sign: String, operands: Vec<Box<Token>> }
}

struct NumberBody {
    raw: String,
    base: u8
}