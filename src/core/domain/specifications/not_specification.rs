use crate::core::domain::specifications::specification::Specification;

pub struct NotSpecification<A> {
    pub spec: A,
}

impl<A> NotSpecification<A> {
    pub fn new(spec: A) -> Self {
        NotSpecification { spec }
    }
}

impl<T, A> Specification<T> for NotSpecification<A> where A: Specification<T> {
    fn is_satisfied_by(&self, candidate: &T) -> bool {
        !self.spec.is_satisfied_by(candidate)
    }
}
