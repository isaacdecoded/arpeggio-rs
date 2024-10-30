use crate::core::domain::specifications::and_specification::AndSpecification;
use crate::core::domain::specifications::not_specification::NotSpecification;
use crate::core::domain::specifications::or_specification::OrSpecification;

pub trait Specification<T>: Sized {
    fn is_satisfied_by(&self, t: &T) -> bool;

    fn and<S>(self, specification: S) -> AndSpecification<Self, S> {
        AndSpecification::new(self, specification)
    }

    fn or<S>(self, specification: S) -> OrSpecification<Self, S> {
        OrSpecification::new(self, specification)
    }

    fn not(self) -> NotSpecification<Self> {
        NotSpecification::new(self)
    }
}
