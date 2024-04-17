use std::time::SystemTime;
use std::collections::HashMap;
use std::sync::{ Arc, RwLock };
use crate::{ backoffice::plan::domain::entities::plan::Plan, core::domain::models::entity::Entity };

#[derive(Clone)]
pub struct TodoWriteModel {
    pub id: String,
    pub plan_id: String,
    pub description: String,
    pub status: String,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
}

pub struct PlanWriteModel {
    pub name: String,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
}

pub struct TodoReadModel {
    pub id: String,
    pub description: String,
    pub status: String,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
}

pub struct PlanReadModel {
    pub id: String,
    pub name: String,
    pub todos: Vec<TodoReadModel>,
    pub created_at: SystemTime,
    pub updated_at: Option<SystemTime>,
}

pub struct InMemoryRepository {
    pub read_plans: Arc<RwLock<HashMap<String, PlanReadModel>>>,
    pub write_plans: Arc<RwLock<HashMap<String, PlanWriteModel>>>,
    pub write_todos: Arc<RwLock<HashMap<String, Vec<TodoWriteModel>>>>,
}

impl InMemoryRepository {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            read_plans: Arc::new(RwLock::new(HashMap::new())),
            write_plans: Arc::new(RwLock::new(HashMap::new())),
            write_todos: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    pub fn sync_read_plans(&self, plan: &Plan) {
        self.read_plans
            .write()
            .unwrap()
            .insert(plan.get_id().to_string(), PlanReadModel {
                id: plan.get_id().to_string(),
                name: plan.get_name().to_string(),
                todos: plan
                    .get_todos()
                    .iter()
                    .map(|todo| TodoReadModel {
                        id: todo.get_id().to_string(),
                        description: todo.get_description().to_string(),
                        status: todo.get_status().to_string(),
                        created_at: *todo.get_created_at(),
                        updated_at: todo.get_updated_at().copied(),
                    })
                    .collect(),
                created_at: *plan.get_created_at(),
                updated_at: plan.get_updated_at().copied(),
            });
    }
}
