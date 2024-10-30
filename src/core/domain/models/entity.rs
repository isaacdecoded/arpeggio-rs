use crate::core::domain::models::value_object::ValueObject;
use std::time::SystemTime;

pub trait Entity<Id: ValueObject<String>> {
    fn get_id(&self) -> &Id;
    fn get_created_at(&self) -> &SystemTime;
    fn get_updated_at(&self) -> Option<&SystemTime>;
    fn update(&mut self);
}
