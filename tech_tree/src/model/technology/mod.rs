use crate::model::technology::name::TechnologyName;

pub mod name;
pub mod tree;

#[derive(Clone, Debug, PartialEq)]
pub struct TechnologyId(usize);

#[derive(Clone, Debug, PartialEq)]
pub struct Technology {
    id: TechnologyId,
    name: TechnologyName,
    predecessors: Vec<TechnologyId>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Input {
    name: String,
    predecessors: Vec<String>,
}
