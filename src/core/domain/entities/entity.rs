use chrono::{Local, DateTime};
use crate::core::domain::entities::{
    value_object::ValueObject,
    string_value_object::StringValueObject,
    date_value_object::DateValueObject,
};

#[derive(Clone)]
pub struct Entity {
    id: StringValueObject,
    created_at: DateValueObject,
    updated_at: Option<DateValueObject>,
}

impl Entity {
    pub fn new(
        id: StringValueObject,
        created_at: Option<DateValueObject>,
        updated_at: Option<DateValueObject>,
    ) -> Entity {
        let mut created_date = DateValueObject::new(Local::now());
        if let Some(created_at) = created_at {
            created_date = created_at
        }
        Entity {
            id,
            created_at: created_date,
            updated_at: if let Some(updated_at) = updated_at {
                Some(updated_at)
            } else {
                None
            },
        }
    }

    pub fn id(&self) -> String {
        self.id.value()
    }

    pub fn created_at(&self) -> DateTime<Local> {
        self.created_at.value()
    }

    pub fn updated_at(&self) -> Option<DateTime<Local>> {
        if let Some(updated_at) = &self.updated_at {
            return Some(updated_at.value())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::domain::entities::entity::Entity;
    use crate::core::domain::entities::{
        value_object::ValueObject,
        string_value_object::StringValueObject
    };

    #[test]
    fn should_initialize_valid_instance() {
        let id = StringValueObject::new("id".to_string());
        let entity = Entity::new(id, None, None);
        assert_eq!(entity.id(), "id".to_string());
        assert!(Some(entity.created_at.value()).is_some());
        assert!(entity.updated_at.is_none());
    }

    #[test]
    fn should_initialize_valid_instance_with_updated_at() {
        let id = StringValueObject::new("id".to_string());
        let date = chrono::Local::now();

        let entity = Entity::new(id, None, Some(ValueObject::new(date)));
        assert!(entity.updated_at.is_some());
    }
}
