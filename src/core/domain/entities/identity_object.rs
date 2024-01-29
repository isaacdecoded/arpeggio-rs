use crate::core::domain::entities::value_object::ValueObject;

#[derive(Clone)]
pub struct IdentityObject {
    value: String,
}

impl ValueObject<String> for IdentityObject {
    fn new(value: String) -> Self {
        Self { value }
    }

    fn get_value(&self) -> &String {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use crate::core::domain::entities::{
        value_object::ValueObject,
        identity_object::IdentityObject,
    };

    #[test]
    fn should_initialize_valid_instance() {
        let value = "str value".to_string();
        let vo = IdentityObject::new(value);
        assert_eq!(vo.get_value().to_string(), "str value".to_string());
    }
}
