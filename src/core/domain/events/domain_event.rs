use chrono::Local;
use crate::core::domain::entities::{
    value_object::ValueObject,
    string_value_object::StringValueObject,
    date_value_object::DateValueObject,
};

#[derive(Clone)]
pub struct DomainEvent {
    aggregate_id: StringValueObject,
    event_name: StringValueObject,
    occurring_time: DateValueObject,
}

impl DomainEvent {
    pub fn new(aggregate_id: StringValueObject, event_name: StringValueObject) -> DomainEvent {
        DomainEvent {
            aggregate_id,
            event_name,
            occurring_time: DateValueObject::new(Local::now()),
        }
    }

    pub fn aggregate_id(&self) -> String {
        self.aggregate_id.value()
    }

    pub fn event_name(&self) -> String {
        self.event_name.value()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::domain::entities::{
        value_object::ValueObject,
        string_value_object::StringValueObject,
    };
    use crate::core::domain::events::domain_event::DomainEvent;

    #[test]
    fn should_initialize_valid_instance() {
        let aggregate_id = StringValueObject::new("id".to_string());
        let event_name = StringValueObject::new("domain event name".to_string());
        let domain_event = DomainEvent::new(
            aggregate_id,
            event_name,
        );
        assert_eq!(domain_event.aggregate_id(), "id".to_string());
        assert_eq!(domain_event.event_name(), "domain event name".to_string());
    }
}
