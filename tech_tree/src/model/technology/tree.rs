use crate::model::technology::{Technology, TechnologyId};

#[derive(Clone, Debug, PartialEq)]
pub struct TechnologyTree {
    technologies: Vec<Technology>,
}

impl TechnologyTree {
    pub fn new(technologies: Vec<Technology>) -> Self {
        TechnologyTree { technologies }
    }

    pub fn technologies(&self) -> &Vec<Technology> {
        &self.technologies
    }

    pub fn get(&self, id: TechnologyId) -> Option<&Technology> {
        self.technologies.get(id.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let tree = TechnologyTree::new(vec![
            Technology::simple(0),
            Technology::simple(1),
            Technology::simple(2),
        ]);

        assert_eq!(tree.get(TechnologyId::new(0)), Some(&Technology::simple(0)));
        assert_eq!(tree.get(TechnologyId::new(1)), Some(&Technology::simple(1)));
        assert_eq!(tree.get(TechnologyId::new(2)), Some(&Technology::simple(2)));
        assert_eq!(tree.get(TechnologyId::new(3)), None);
    }
}
