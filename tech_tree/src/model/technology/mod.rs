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

    pub fn get_predecessor_index(&self, id: TechnologyId) -> Option<usize> {
        for (index, predecessor) in self.predecessors.iter().enumerate() {
            if *predecessor == id {
                return Some(index);
            }
        }

        None
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_technology() {
        let technology = Technology::simple2(2, "Tech", vec![0, 1], vec![3, 4]);

        assert_eq!(technology.name().get_full(), "Tech");
        assert_eq!(technology.id(), &TechnologyId::new(2));
        assert_eq!(
            technology.predecessors(),
            &vec![TechnologyId::new(0), TechnologyId::new(1)]
        );
        assert_eq!(
            technology.successors(),
            &vec![TechnologyId::new(3), TechnologyId::new(4)]
        );
    }

    #[test]
    fn test_get_predecessor_index() {
        let technology = Technology::simple2(0, "Tech", vec![2, 3], vec![1]);

        assert_predecessor_index(&technology, 0, None);
        assert_predecessor_index(&technology, 1, None);
        assert_predecessor_index(&technology, 2, Some(0));
        assert_predecessor_index(&technology, 3, Some(1));
        assert_predecessor_index(&technology, 4, None);
    }

    fn assert_predecessor_index(technology: &Technology, id: usize, result: Option<usize>) {
        assert_eq!(
            technology.get_predecessor_index(TechnologyId::new(id)),
            result
        );
    }

    #[test]
    fn test_input() {
        let input = Input::test("A", vec!["B", "C"]);

        assert_eq!(input.name(), "A");
        assert_eq!(input.predecessors(), &vec!["B", "C"]);
    }
}
