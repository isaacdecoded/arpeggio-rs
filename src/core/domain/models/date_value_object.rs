use crate::core::domain::models::value_object::ValueObject;
use std::{error::Error, time::SystemTime};

#[derive(Copy, Clone, PartialEq)]
pub struct DateValueObject {
    value: SystemTime,
}

impl ValueObject<SystemTime> for DateValueObject {
    fn new(value: SystemTime) -> Result<Self, Box<dyn Error + Sync + Send>> {
        Ok(DateValueObject { value })
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
        DateValueObject::new(SystemTime::now()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::domain::models::{
        date_value_object::DateValueObject, value_object::ValueObject,
    };
    use std::time::SystemTime;

    #[test]
    fn should_initialize_valid_instance() {
        let value = SystemTime::now();
        let vo = DateValueObject::new(value).unwrap();
        assert_eq!(vo.get_value().to_owned(), value);
    }
}
