use crate::core::domain::entities::identity_object::IdentityObject;

pub trait Entity {
    fn get_id(&self) -> &IdentityObject;
}
