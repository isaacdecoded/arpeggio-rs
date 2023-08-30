use crate::core::domain::entities::value_object::ValueObject;

#[derive(Clone)]
pub struct StringValueObject {
    value: String
}

impl ValueObject<String> for StringValueObject {
    fn new(value: String) -> Self {
        StringValueObject { value }
    }

    fn value(&self) -> String {
        self.value.to_string()
    }
}

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