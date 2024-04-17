use std::time::SystemTime;
use crate::core::domain::models::value_object::ValueObject;

#[derive(Copy, Clone, PartialEq)]
pub struct DateValueObject {
    value: SystemTime,
}

impl ValueObject<SystemTime> for DateValueObject {
    fn new(value: SystemTime) -> Self {
        DateValueObject { value }
    }

    fn get_value(&self) -> &SystemTime {
        &self.value
    }

    fn is_equal(&self, other: &Self) -> bool {
        self.value == *other.get_value()
    }
}

impl DateValueObject {
    pub fn now() -> Self {
        DateValueObject::new(SystemTime::now())
    }
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;
    use crate::core::domain::models::{
        value_object::ValueObject,
        date_value_object::DateValueObject,
    };

    #[test]
    fn should_initialize_valid_instance() {
        let value = SystemTime::now();
        let vo = DateValueObject::new(value);
        assert_eq!(vo.get_value().to_owned(), value);
    }
}
