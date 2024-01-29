use chrono::{ Local, DateTime };
use crate::core::domain::entities::value_object::ValueObject;

#[derive(Copy, Clone, PartialEq)]
pub struct DateValueObject {
    value: DateTime<Local>,
}

impl ValueObject<DateTime<Local>> for DateValueObject {
    fn new(value: DateTime<Local>) -> Self {
        DateValueObject { value }
    }

    fn get_value(&self) -> &DateTime<Local> {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use chrono::Local;
    use crate::core::domain::entities::{
        value_object::ValueObject,
        date_value_object::DateValueObject,
    };

    #[test]
    fn should_initialize_valid_instance() {
        let value = Local::now();
        let vo = DateValueObject::new(value);
        // assert_eq!(vo.get_value(), value);
    }
}
