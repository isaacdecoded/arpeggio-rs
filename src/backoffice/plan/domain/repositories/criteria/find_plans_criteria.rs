use crate::core::domain::repositories::criteria::{Criteria, Filter, Sort};

pub enum PlanFieldEnum {
    Name,
}

pub struct FindPlansCriteria {
    filters: Vec<Filter<PlanFieldEnum>>,
    selections: Option<Vec<PlanFieldEnum>>,
    sorts: Option<Vec<Sort<PlanFieldEnum>>>,
    limit: Option<u16>,
    offset: Option<u16>,
}

impl FindPlansCriteria {
    pub fn with_limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn with_offset(mut self, offset: u16) -> Self {
        self.offset = Some(offset);
        self
    }
}

impl Criteria<PlanFieldEnum> for FindPlansCriteria {
    fn new(filters: Vec<Filter<PlanFieldEnum>>) -> Self {
        Self {
            filters,
            selections: None,
            sorts: None,
            limit: None,
            offset: None,
        }
    }

    fn get_filters(&self) -> &[Filter<PlanFieldEnum>] {
        &self.filters
    }

    fn get_selections(&self) -> Option<&[PlanFieldEnum]> {
        self.selections.as_deref()
    }

    fn get_sorts(&self) -> Option<&[Sort<PlanFieldEnum>]> {
        self.sorts.as_deref()
    }

    fn get_limit(&self) -> Option<&u16> {
        self.limit.as_ref()
    }

    fn get_offset(&self) -> Option<&u16> {
        self.offset.as_ref()
    }
}
