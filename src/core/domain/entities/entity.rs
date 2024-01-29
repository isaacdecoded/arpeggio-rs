use crate::core::domain::entities::identity_object::IdentityObject;

pub trait Entity {
    fn get_id(&self) -> &IdentityObject;
}

pub trait Creatable<Props>: Entity {
    fn create(props: Props) -> Self;
    // fn get_created_at(&self) -> &DateValueObject;
}

pub trait Updatable<Props>: Entity {
    fn update(&mut self, props: Props);
    // fn get_updated_at(&self) -> &DateValueObject;
}

pub trait Recreatable<Props>: Entity {
    fn recreate(props: Props) -> Self;
    // fn get_created_at(&self) -> &DateValueObject;
    // fn get_updated_at(&self) -> &DateValueObject;
}
