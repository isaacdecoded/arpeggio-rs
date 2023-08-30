pub trait ValueObject<T> {
    fn new(value: T) -> Self;
    fn value(&self) -> T;
}
