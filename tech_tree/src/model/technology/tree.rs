use crate::model::error::AddError;
use crate::model::technology::name::TechnologyName;
use crate::model::technology::{Input, Technology, TechnologyId};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct TechnologyTree {
    technologies: Vec<Technology>,
}

impl TechnologyTree {
    pub fn create(technologies: Vec<Input>) -> Result<Self, AddError> {
        let name_to_id = Self::create_name_to_id_map(&technologies)?;

        let result: Result<Vec<Technology>, AddError> = technologies
            .into_iter()
            .enumerate()
            .map(|(id, t)| Self::convert_technology(t, id, &name_to_id))
            .collect();

        Ok(TechnologyTree {
            technologies: result?,
        })
    }

    fn create_name_to_id_map(technologies: &[Input]) -> Result<HashMap<String, usize>, AddError> {
        let mut name_to_id = HashMap::new();

        for technology in technologies {
            if name_to_id
                .insert(technology.name.clone(), name_to_id.len())
                .is_some()
            {
                return Err(AddError::NameExists(technology.name.clone()));
            }
        }

        Ok(name_to_id)
    }

    fn convert_technology(
        technology: Input,
        id: usize,
        name_to_id: &HashMap<String, usize>,
    ) -> Result<Technology, AddError> {
        let requirements: Result<Vec<TechnologyId>, AddError> = technology
            .predecessors
            .iter()
            .map(|name| Self::convert_name(name, name_to_id))
            .collect();

        Ok(Technology {
            id: TechnologyId(id),
            name: TechnologyName::new(technology.name)?,
            predecessors: requirements?,
        })
    }

    fn convert_name(
        name: &str,
        name_to_id: &HashMap<String, usize>,
    ) -> Result<TechnologyId, AddError> {
        name_to_id
            .get(name)
            .map(|id| TechnologyId(*id))
            .ok_or_else(|| AddError::UnknownPredecessor(name.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tree() {
        let input = vec![
            Input {
                name: "t0".to_string(),
                predecessors: vec![],
            },
            Input {
                name: "t1".to_string(),
                predecessors: vec![],
            },
            Input {
                name: "t2".to_string(),
                predecessors: vec!["t0".to_string(), "t1".to_string()],
            },
            Input {
                name: "t3".to_string(),
                predecessors: vec!["t2".to_string()],
            },
            Input {
                name: "t4".to_string(),
                predecessors: vec!["t2".to_string()],
            },
        ];

        assert_eq!(
            TechnologyTree::create(input),
            Ok(TechnologyTree {
                technologies: vec![
                    Technology {
                        id: TechnologyId(0),
                        name: TechnologyName::Simple("t0".to_string()),
                        predecessors: vec![],
                    },
                    Technology {
                        id: TechnologyId(1),
                        name: TechnologyName::Simple("t1".to_string()),
                        predecessors: vec![],
                    },
                    Technology {
                        id: TechnologyId(2),
                        name: TechnologyName::Simple("t2".to_string()),
                        predecessors: vec![TechnologyId(0), TechnologyId(1)],
                    },
                    Technology {
                        id: TechnologyId(3),
                        name: TechnologyName::Simple("t3".to_string()),
                        predecessors: vec![TechnologyId(2)],
                    },
                    Technology {
                        id: TechnologyId(4),
                        name: TechnologyName::Simple("t4".to_string()),
                        predecessors: vec![TechnologyId(2)],
                    },
                ]
            })
        );
    }
}
