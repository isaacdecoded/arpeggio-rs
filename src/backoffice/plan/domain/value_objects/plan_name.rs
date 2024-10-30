use std::error::Error;

use crate::core::domain::models::value_object::ValueObject;

#[derive(Clone, Debug)]
pub struct PlanName {
    value: String,
}

impl PlanName {
    const MAX_LENGTH: usize = 512;
}

impl ValueObject<String> for PlanName {
    fn new(value: String) -> Result<Self, Box<dyn Error + Sync + Send>> {
        if value.len() > Self::MAX_LENGTH {
            return Err(format!(
                "The name exceeds the maximum length of {} characters.",
                Self::MAX_LENGTH
            )
            .into());
        }
        Ok(Self { value })
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
    use crate::backoffice::plan::domain::value_objects::plan_name::PlanName;
    use crate::core::domain::models::value_object::ValueObject;

    #[test]
    fn should_initialize_valid_instance() {
        let value = "Lorem Ipsum";
        let vo = PlanName::new(value.to_string()).unwrap();
        assert_eq!(vo.get_value(), value);
    }

    #[test]
    fn should_refuse_invalid_instance() {
        let value = "Lorem Ipsum".repeat(50);
        let vo = PlanName::new(value.to_string());
        assert!(vo.is_err());
    }
}
