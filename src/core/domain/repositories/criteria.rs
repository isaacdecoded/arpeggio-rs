use std::mem::ManuallyDrop;
use chrono::{ Local, DateTime };

pub enum FilterOperator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    Contains,
    NotContains,
}

pub enum SortOrder {
    Asc,
    Desc,
}

pub struct Sort {
    pub field: String,
    pub order: SortOrder,
}

pub struct Filter {
    pub field: String,
    pub operator: FilterOperator,
    pub value: String,
}

pub struct Criteria {
    pub filters: Vec<Filter>,
    pub limit: Option<u16>,
    pub offset: Option<u16>,
}

impl Criteria {
    pub fn new(filters: Vec<Filter>, limit: Option<u16>, offset: Option<u16>) -> Self {
        Self {
            filters,
            limit,
            offset,
        }
    }

    pub fn new_with_filters_only(filters: Vec<Filter>) -> Self {
        Self {
            filters,
            limit: None,
            offset: None,
        }
    }
}
