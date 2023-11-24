use crate::core::domain::entities::{
    value_object::ValueObject,
    date_value_object::DateValueObject,
};

pub trait Newable<Props> {
    fn new(props: Props) -> Self;
}

pub trait Updatable<Props> {
    fn update(&mut self, props: Props);
}

pub trait Recreable<Props> {
    fn recreate(props: Props) -> Self;
}

pub trait Entity {
    type Id: ValueObject<String>;

    fn id(&self) -> &Self::Id;
    fn created_at(&self) -> &DateValueObject;
    fn updated_at(&self) -> &Option<DateValueObject>;
}