use serde::{Deserialize, Serialize};
use tech_tree::model::technology::tree::TechnologyTree;
use tech_tree::model::technology::{Input, Technology};

pub mod tree;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TechnologyDefinition {
    name: String,
    predecessors: Vec<String>,
}

impl TechnologyDefinition {
    pub fn new(name: String, predecessors: Vec<String>) -> Self {
        TechnologyDefinition { name, predecessors }
    }

    pub fn to_model(self) -> Input {
        Input::new(self.name, self.predecessors)
    }

    pub fn from_model(technology: &Technology, tree: &TechnologyTree) -> TechnologyDefinition {
        TechnologyDefinition::new(
            technology.name().get_full().to_string(),
            technology
                .predecessors()
                .iter()
                .map(|id| {
                    tree.get(*id)
                        .map(|technology| technology.name().get_full().to_string())
                        .unwrap_or_else(|| "UNKNOWN".to_string())
                })
                .collect(),
        )
    }
}
