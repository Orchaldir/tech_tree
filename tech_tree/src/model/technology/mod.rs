use crate::model::technology::name::TechnologyName;

pub mod name;
pub mod tree;

#[derive(Clone, Debug, PartialEq)]
pub struct TechnologyId(usize);

#[derive(Clone, Debug, PartialEq)]
pub struct Technology<T> {
    name: TechnologyName,
    requirements: Vec<T>,
}
