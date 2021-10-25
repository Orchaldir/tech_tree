use crate::model::technology::Technology;

#[derive(Clone, Debug, PartialEq)]
pub struct TechnologyTree {
    technologies: Vec<Technology>,
}

impl TechnologyTree {
    pub fn new(technologies: Vec<Technology>) -> Self {
        TechnologyTree { technologies }
    }
}
