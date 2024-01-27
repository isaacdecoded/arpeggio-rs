use chrono::{ Local, DateTime };

pub trait DEvent: Send + Sync {
    fn get_name(&self) -> String;
    fn get_aggregate_root_id(&self) -> String;
    fn get_occurring_time(&self) -> DateTime<Local>;
}

#[derive(Clone)]
pub struct DomainEvent {
    name: String,
    aggregate_root_id: String,
    occurring_time: DateTime<Local>,
}

impl DomainEvent {
    pub fn new(name: String, aggregate_root_id: String) -> DomainEvent {
        DomainEvent {
            name,
            aggregate_root_id,
            occurring_time: Local::now(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_aggregate_root_id(&self) -> String {
        self.aggregate_root_id.to_string()
    }

    pub fn get_occurring_time(&self) -> DateTime<Local> {
        self.occurring_time
    }
}

#[cfg(test)]
mod tests {
    use crate::core::domain::events::domain_event::DomainEvent;

    #[test]
    fn should_initialize_valid_instance() {
        let aggregate_root_id = "id".to_string();
        let name = "domain event name".to_string();
        let domain_event = DomainEvent::new(name, aggregate_root_id);
        assert_eq!(domain_event.get_aggregate_root_id(), "id".to_string());
        assert_eq!(domain_event.get_name(), "domain event name".to_string());
    }
}
