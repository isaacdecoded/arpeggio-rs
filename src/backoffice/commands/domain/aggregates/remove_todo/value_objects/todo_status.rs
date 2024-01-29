use crate::core::domain::entities::value_object::ValueObject;

#[derive(Clone, Copy)]
pub enum TodoStatuses {
    ARCHIVED,
    DONE,
    REMOVED,
}

#[derive(Clone)]
pub struct TodoStatus {
    value: TodoStatuses,
}

impl ValueObject<TodoStatuses> for TodoStatus {
    fn new(value: TodoStatuses) -> Self {
        TodoStatus { value }
    }

    fn get_value(&self) -> &TodoStatuses {
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
