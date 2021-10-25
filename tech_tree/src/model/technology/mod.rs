use crate::model::technology::name::TechnologyName;

pub mod name;
pub mod tree;

#[derive(Clone, Debug, PartialEq)]
pub struct TechnologyId(usize);

impl TechnologyId {
    pub fn new(id: usize) -> Self {
        TechnologyId(id)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Technology {
    id: TechnologyId,
    name: TechnologyName,
    predecessors: Vec<TechnologyId>,
}

impl Technology {
    pub fn new(id: TechnologyId, name: TechnologyName, predecessors: Vec<TechnologyId>) -> Self {
        Technology {
            id,
            name,
            predecessors,
        }
    }

    pub fn id(&self) -> &TechnologyId {
        &self.id
    }

    pub fn name(&self) -> &TechnologyName {
        &self.name
    }

    pub fn predecessors(&self) -> &Vec<TechnologyId> {
        &self.predecessors
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Input {
    name: String,
    predecessors: Vec<String>,
}

impl Input {
    pub fn new(name: String, predecessors: Vec<String>) -> Self {
        Input { name, predecessors }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn predecessors(&self) -> &Vec<String> {
        &self.predecessors
    }
}
