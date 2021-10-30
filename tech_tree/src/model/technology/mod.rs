use crate::model::technology::name::TechnologyName;

pub mod name;
pub mod tree;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TechnologyId(usize);

impl TechnologyId {
    pub fn new(id: usize) -> Self {
        TechnologyId(id)
    }

    pub fn id(&self) -> usize {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Technology {
    id: TechnologyId,
    name: TechnologyName,
    predecessors: Vec<TechnologyId>,
    successors: Vec<TechnologyId>,
}

impl Technology {
    pub fn new(
        id: TechnologyId,
        name: TechnologyName,
        predecessors: Vec<TechnologyId>,
        successors: Vec<TechnologyId>,
    ) -> Self {
        Technology {
            id,
            name,
            predecessors,
            successors,
        }
    }

    pub fn simple(id: usize) -> Self {
        Technology {
            id: TechnologyId(id),
            name: TechnologyName::Simple(format!("Tech {}", id)),
            predecessors: Vec::new(),
            successors: Vec::new(),
        }
    }

    pub fn simple2(
        id: usize,
        name: &str,
        predecessors: Vec<usize>,
        successors: Vec<usize>,
    ) -> Self {
        Technology {
            id: TechnologyId(id),
            name: TechnologyName::Simple(name.to_string()),
            predecessors: predecessors.into_iter().map(TechnologyId::new).collect(),
            successors: successors.into_iter().map(TechnologyId::new).collect(),
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

    pub fn successors(&self) -> &Vec<TechnologyId> {
        &self.successors
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

    pub fn test(name: &str, predecessors: Vec<&str>) -> Self {
        Input {
            name: name.to_string(),
            predecessors: predecessors.into_iter().map(|p| p.to_string()).collect(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn predecessors(&self) -> &Vec<String> {
        &self.predecessors
    }
}
