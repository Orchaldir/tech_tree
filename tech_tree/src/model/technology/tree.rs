use crate::model::technology::name::TechnologyName;
use crate::model::technology::{Technology, TechnologyId};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct TechnologyTree {
    technologies: Vec<Technology<TechnologyId>>,
}

impl TechnologyTree {
    pub fn create(technologies: Vec<Technology<TechnologyName>>) -> Option<Self> {
        let name_to_id = Self::create_name_to_id_map(&technologies)?;

        let result: Option<Vec<Technology<TechnologyId>>> = technologies
            .into_iter()
            .map(|t| Self::convert_technology(t, &name_to_id))
            .collect();

        Some(TechnologyTree {
            technologies: result?,
        })
    }

    fn create_name_to_id_map(
        technologies: &[Technology<TechnologyName>],
    ) -> Option<HashMap<String, usize>> {
        let mut name_to_id = HashMap::new();

        for technology in technologies {
            if name_to_id
                .insert(technology.name.get_full().to_string(), name_to_id.len())
                .is_some()
            {
                return None;
            }
        }

        Some(name_to_id)
    }

    fn convert_technology(
        technology: Technology<TechnologyName>,
        name_to_id: &HashMap<String, usize>,
    ) -> Option<Technology<TechnologyId>> {
        let requirements: Option<Vec<TechnologyId>> = technology
            .requirements
            .iter()
            .map(|name| Self::convert_name(name, name_to_id))
            .collect();

        Some(Technology {
            name: technology.name,
            requirements: requirements?,
        })
    }

    fn convert_name(
        name: &TechnologyName,
        name_to_id: &HashMap<String, usize>,
    ) -> Option<TechnologyId> {
        name_to_id.get(name.get_full()).map(|id| TechnologyId(*id))
    }
}
