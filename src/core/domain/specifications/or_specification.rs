use crate::core::domain::specifications::specification::Specification;

pub struct OrSpecification<A, B> {
    spec1: A,
    spec2: B,
}

impl<A, B> OrSpecification<A, B> {
    pub fn new(spec1: A, spec2: B) -> Self {
        OrSpecification { spec1, spec2 }
    }
}

impl<T, A, B> Specification<T>
    for OrSpecification<A, B>
    where A: Specification<T>, B: Specification<T>
{
    fn is_satisfied_by(&self, candidate: &T) -> bool {
        self.spec1.is_satisfied_by(candidate) || self.spec2.is_satisfied_by(candidate)
    }
}
