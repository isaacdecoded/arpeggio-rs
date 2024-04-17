use crate::core::domain::models::value_object::ValueObject;

#[derive(Clone)]
pub struct TodoDescription {
    value: String,
}

impl TodoDescription {
    const MAX_LENGTH: usize = 1200;
}

impl ValueObject<String> for TodoDescription {
    fn new(value: String) -> Self {
        if value.len() > Self::MAX_LENGTH {
            panic!(
                "The description exceeds the maximum length of {} characters.",
                Self::MAX_LENGTH
            );
        }
        TodoDescription { value }
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
