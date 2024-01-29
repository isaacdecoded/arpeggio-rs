use crate::core::domain::entities::value_object::ValueObject;

#[derive(Clone)]
pub struct TodoName {
    value: String,
}

impl ValueObject<String> for TodoName {
    fn new(value: String) -> Self {
        if value.len() > 500 {
            panic!("The name exceeds the maximum length of 500 characters.");
        }
        TodoName { value }
    }

    fn get_value(&self) -> &String {
        &self.value
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
