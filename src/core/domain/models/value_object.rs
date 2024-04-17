use std::error::Error;

pub trait ValueObject<T>: Sized {
    fn new(value: T) -> Result<Self, Box<dyn Error + Sync + Send>>;
    fn get_value(&self) -> &T;
    fn is_equal(&self, other: &Self) -> bool;
}
