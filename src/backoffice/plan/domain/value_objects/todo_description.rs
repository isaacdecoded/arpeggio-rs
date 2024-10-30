use std::error::Error;

use crate::core::domain::models::value_object::ValueObject;

#[derive(Clone)]
pub struct TodoDescription {
    value: String,
}

impl TodoDescription {
    const MAX_LENGTH: usize = 1200;
}

impl ValueObject<String> for TodoDescription {
    fn new(value: String) -> Result<Self, Box<dyn Error + Sync + Send>> {
        if value.len() > Self::MAX_LENGTH {
            return Err(format!(
                "The description exceeds the maximum length of {} characters.",
                Self::MAX_LENGTH
            )
            .into());
        }
        Ok(TodoDescription { value })
    }

    fn get_value(&self) -> &String {
        &self.value
    }

    fn is_equal(&self, other: &Self) -> bool {
        self.value == *other.get_value()
    }
}

#[cfg(test)]
mod tests {
    use crate::backoffice::plan::domain::value_objects::todo_description::TodoDescription;
    use crate::core::domain::models::value_object::ValueObject;

    #[test]
    fn should_initialize_valid_instance() {
        let value = "Lorem Ipsum";
        let vo = TodoDescription::new(value.to_string()).unwrap();
        assert_eq!(vo.get_value(), value);
    }

    #[test]
    fn should_refuse_invalid_instance() {
        let value = "Lorem Ipsum".repeat(110);
        let vo = TodoDescription::new(value.to_string());
        assert!(vo.is_err());
    }
}
