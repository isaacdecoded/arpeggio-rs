use std::mem::ManuallyDrop;
use chrono::{Local, DateTime};

pub enum FilterOperator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    Contains,
    NotContains,
}

pub union FilterValue {
    pub i: i32,
    pub s: ManuallyDrop<String>,
    pub b: bool,
    pub d: DateTime<Local>,
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
    pub value: FilterValue,
}

pub struct Criteria {
    pub filters: Vec<Filter>,
    pub selection: Option<String>,
    pub limit: Option<u16>,
    pub offset: Option<u16>,
}

impl Criteria {
    pub fn new(
        filters: Vec<Filter>,
        selection: Option<String>,
        limit: Option<u16>,
        offset: Option<u16>,
    ) -> Self {
        Self {
            filters,
            selection,
            limit,
            offset,
        }
    }

    pub fn new_with_filters_only(filters: Vec<Filter>) -> Self {
        Self {
            filters,
            selection: None,
            limit: None,
            offset: None,
        }
    }
}

fn a() {
    let f = Filter{
        field: "name".to_string(),
        operator: FilterOperator::Equal,
        value: FilterValue{
            s: ManuallyDrop::new("Isaac".to_string())
        },
    };
    let c = Criteria::new(vec![f], None, None, None);
    for filter in c.filters {
        unsafe {
            match filter.value {
                FilterValue { i } => (),
                FilterValue { s } => (),
                FilterValue { b } => (),
                FilterValue { d } => (),
                _ => {}
            }
        }
        match filter.operator {
            FilterOperator::Contains => {}
            FilterOperator::Equal => todo!(),
            FilterOperator::NotEqual => todo!(),
            FilterOperator::GreaterThan => todo!(),
            FilterOperator::LessThan => todo!(),
            FilterOperator::NotContains => todo!(),
        };
    }
}