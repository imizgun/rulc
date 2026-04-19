pub trait Parsable<T> {
    fn parse(str: &str) -> Option<T>;
}