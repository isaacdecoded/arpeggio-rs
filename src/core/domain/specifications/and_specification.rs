use crate::core::domain::specifications::specification::Specification;

pub struct AndSpecification<A, B> {
    pub spec1: A,
    pub spec2: B,
}

impl<A, B> AndSpecification<A, B> {
    pub fn new(spec1: A, spec2: B) -> Self {
        AndSpecification { spec1, spec2 }
    }
}

impl<T, A, B> Specification<T> for AndSpecification<A, B>
where
    A: Specification<T>,
    B: Specification<T>,
{
    fn is_satisfied_by(&self, candidate: &T) -> bool {
        self.spec1.is_satisfied_by(candidate) && self.spec2.is_satisfied_by(candidate)
    }
}
