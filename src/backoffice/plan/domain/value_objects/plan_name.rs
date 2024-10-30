use crate::core::domain::models::value_object::ValueObject;

#[derive(Clone)]
pub struct PlanName {
    value: String,
}

impl ValueObject<String> for PlanName {
    fn new(value: String) -> Self {
        if value.len() > 500 {
            panic!("The name exceeds the maximum length of 500 characters.");
        }
        Self { value }
    }

    fn get_value(&self) -> &String {
        &self.value
    }

    fn is_equal(&self, other: &Self) -> bool {
        self.value == *other.get_value()
    }
}

/*
#[cfg(test)]
mod tests {
    use crate::core::domain::entities::{
        value_object::ValueObject,
        string_value_object::StringValueObject,
    };

    #[test]
    fn should_initialize_valid_instance() {
        let value = "str value".to_string();
        let vo = StringValueObject::new(value);
        assert_eq!(vo.value(), "str value".to_string());
    }
}
*/
