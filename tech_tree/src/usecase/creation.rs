use crate::model::error::AddError;
use crate::model::technology::name::TechnologyName;
use crate::model::technology::tree::TechnologyTree;
use crate::model::technology::{Input, Technology, TechnologyId};
use std::collections::HashMap;

pub fn create_tree(technologies: Vec<Input>) -> Result<TechnologyTree, AddError> {
    let name_to_id = create_name_to_id_map(&technologies)?;

    let result: Result<Vec<Technology>, AddError> = technologies
        .into_iter()
        .enumerate()
        .map(|(id, t)| convert_technology(t, id, &name_to_id))
        .collect();

    Ok(TechnologyTree::new(result?))
}

fn create_name_to_id_map(technologies: &[Input]) -> Result<HashMap<String, usize>, AddError> {
    let mut name_to_id = HashMap::new();

    for technology in technologies {
        if name_to_id
            .insert(technology.name().to_string(), name_to_id.len())
            .is_some()
        {
            return Err(AddError::NameExists(technology.name().to_string()));
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
        .predecessors()
        .iter()
        .map(|name| convert_name(name, name_to_id))
        .collect();

    Ok(Technology::new(
        TechnologyId::new(id),
        TechnologyName::new(technology.name())?,
        requirements?,
    ))
}

fn convert_name(name: &str, name_to_id: &HashMap<String, usize>) -> Result<TechnologyId, AddError> {
    name_to_id
        .get(name)
        .map(|id| TechnologyId::new(*id))
        .ok_or_else(|| AddError::UnknownPredecessor(name.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tree() {
        let input = vec![
            Input::new("t0".to_string(), vec![]),
            Input::new("t1".to_string(), vec![]),
            Input::new("t2".to_string(), vec!["t0".to_string(), "t1".to_string()]),
            Input::new("t3".to_string(), vec!["t2".to_string()]),
            Input::new("t4".to_string(), vec!["t2".to_string()]),
        ];

        assert_eq!(
            create_tree(input),
            Ok(TechnologyTree::new(vec![
                Technology::new(
                    TechnologyId::new(0),
                    TechnologyName::Simple("t0".to_string()),
                    vec![]
                ),
                Technology::new(
                    TechnologyId::new(1),
                    TechnologyName::Simple("t1".to_string()),
                    vec![]
                ),
                Technology::new(
                    TechnologyId::new(2),
                    TechnologyName::Simple("t2".to_string()),
                    vec![TechnologyId::new(0), TechnologyId::new(1)]
                ),
                Technology::new(
                    TechnologyId::new(3),
                    TechnologyName::Simple("t3".to_string()),
                    vec![TechnologyId::new(2)]
                ),
                Technology::new(
                    TechnologyId::new(4),
                    TechnologyName::Simple("t4".to_string()),
                    vec![TechnologyId::new(2)]
                ),
            ]))
        );
    }

    #[test]
    fn test_wrong_order() {
        let input = vec![
            Input::new("t0".to_string(), vec!["t1".to_string()]),
            Input::new("t1".to_string(), vec![]),
        ];

        assert_eq!(
            create_tree(input),
            Ok(TechnologyTree::new(vec![
                Technology::new(
                    TechnologyId::new(0),
                    TechnologyName::Simple("t0".to_string()),
                    vec![TechnologyId::new(1)]
                ),
                Technology::new(
                    TechnologyId::new(1),
                    TechnologyName::Simple("t1".to_string()),
                    vec![]
                )
            ]))
        );
    }

    #[test]
    fn test_invalid_name() {
        let name = "   ";
        let input = vec![Input::new(name.to_string(), vec![])];

        assert_eq!(
            create_tree(input),
            Err(AddError::InvalidName(name.to_string()))
        );
    }

    #[test]
    fn test_duplicate_name() {
        let name = "duplicate";
        let input = vec![
            Input::new(name.to_string(), vec![]),
            Input::new(name.to_string(), vec![]),
        ];

        assert_eq!(
            create_tree(input),
            Err(AddError::NameExists(name.to_string()))
        );
    }

    #[test]
    fn test_unknown_predecessor() {
        let predecessor = "unknown";
        let input = vec![Input::new(
            "name".to_string(),
            vec![predecessor.to_string()],
        )];

        assert_eq!(
            create_tree(input),
            Err(AddError::UnknownPredecessor(predecessor.to_string()))
        );
    }
}
