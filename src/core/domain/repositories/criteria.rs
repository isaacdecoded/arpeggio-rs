use std::{fmt, time::SystemTime};

pub enum FilterOperator {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,
    Contains,
    NotContains,
}

pub enum FilterValue {
    Str(String),
    I32(i32),
    Bool(bool),
    Date(SystemTime),
}

impl fmt::Display for FilterValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FilterValue::Str(s) => write!(f, "{}", s),
            FilterValue::I32(n) => write!(f, "{}", n),
            FilterValue::Bool(b) => write!(f, "{}", b),
            FilterValue::Date(d) => write!(f, "{:?}", d),
        }
    }
}

pub struct Filter<Enum> {
    pub field: Enum,
    pub operator: FilterOperator,
    pub value: FilterValue,
}

pub enum SortOrder {
    Asc,
    Desc,
}

pub struct Sort<Enum> {
    pub field: Enum,
    pub order: SortOrder,
}

pub trait Criteria<Enum> {
    fn new(filters: Vec<Filter<Enum>>) -> Self;
    fn get_filters(&self) -> &[Filter<Enum>];
    fn get_selections(&self) -> Option<&[Enum]>;
    fn get_sorts(&self) -> Option<&[Sort<Enum>]>;
    fn get_limit(&self) -> Option<&u16>;
    fn get_offset(&self) -> Option<&u16>;
}
