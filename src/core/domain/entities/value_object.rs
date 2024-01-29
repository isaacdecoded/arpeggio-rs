pub trait ValueObject<T> {
    fn new(value: T) -> Self;
    fn get_value(&self) -> &T;
}
