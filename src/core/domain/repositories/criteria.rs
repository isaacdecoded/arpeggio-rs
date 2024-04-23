use std::time::SystemTime;

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
    fn get_filters(&self) -> &[&Filter<Enum>];
    fn get_selections(&self) -> Option<&[&Enum]>;
    fn get_sorts(&self) -> Option<&[&Sort<Enum>]>;
    fn get_limit(&self) -> Option<u16>;
    fn get_offset(&self) -> Option<u16>;
}
