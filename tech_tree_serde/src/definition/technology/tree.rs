use crate::definition::technology::TechnologyDefinition;
use serde::{Deserialize, Serialize};
use tech_tree::model::error::AddError;
use tech_tree::model::technology::tree::TechnologyTree;
use tech_tree::usecase::creation::create_tree;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TechnologyTreeDefinition {
    technologies: Vec<TechnologyDefinition>,
}

impl TechnologyTreeDefinition {
    pub fn new(technologies: Vec<TechnologyDefinition>) -> Self {
        TechnologyTreeDefinition { technologies }
    }

    pub fn to_model(self) -> Result<TechnologyTree, AddError> {
        create_tree(
            self.technologies
                .into_iter()
                .map(TechnologyDefinition::to_model)
                .collect(),
        )
    }

    pub fn from_model(tree: &TechnologyTree) -> Self {
        Self::new(
            tree.technologies()
                .iter()
                .map(|technology| TechnologyDefinition::from_model(technology, tree))
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_converting() {
        let definition = TechnologyTreeDefinition::new(vec![
            TechnologyDefinition::new("t0".to_string(), vec![]),
            TechnologyDefinition::new("t1".to_string(), vec!["t0".to_string()]),
            TechnologyDefinition::new("t2".to_string(), vec!["t0".to_string(), "t1".to_string()]),
        ]);

        assert_eq!(
            TechnologyTreeDefinition::from_model(&definition.clone().to_model().unwrap()),
            definition
        )
    }
}
