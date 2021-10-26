use crate::model::error::AddError;
use crate::model::technology::name::TechnologyName;
use crate::model::technology::tree::TechnologyTree;
use crate::model::technology::{Input, Technology, TechnologyId};
use itertools::izip;
use std::collections::HashMap;

pub fn create_tree(input_list: Vec<Input>) -> Result<TechnologyTree, AddError> {
    let name_to_id = create_name_to_id_map(&input_list)?;
    let predecessors_list = process_predecessors(&input_list, &name_to_id)?;
    let successors_list = process_successors(&predecessors_list);
    let mut technologies = Vec::new();

    for (input, predecessors, successors) in izip!(input_list, predecessors_list, successors_list) {
        let technology = Technology::new(
            TechnologyId::new(technologies.len()),
            TechnologyName::new(input.name())?,
            predecessors,
            successors,
        );
        technologies.push(technology);
    }

    Ok(TechnologyTree::new(technologies))
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

fn process_predecessors(
    technologies: &[Input],
    name_to_id: &HashMap<String, usize>,
) -> Result<Vec<Vec<TechnologyId>>, AddError> {
    technologies
        .iter()
        .map(|technology| {
            technology
                .predecessors()
                .iter()
                .map(|name| into_id(name, name_to_id))
                .collect()
        })
        .collect()
}

fn process_successors(predecessors_list: &[Vec<TechnologyId>]) -> Vec<Vec<TechnologyId>> {
    let mut successors: Vec<Vec<TechnologyId>> = vec![Vec::new(); predecessors_list.len()];

    for (id, predecessors) in predecessors_list.iter().enumerate() {
        for predecessor in predecessors {
            successors
                .get_mut(predecessor.id())
                .unwrap()
                .push(TechnologyId::new(id));
        }
    }

    successors
}

fn into_id(name: &str, name_to_id: &HashMap<String, usize>) -> Result<TechnologyId, AddError> {
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
                Technology::simple2(0, "t0", vec![], vec![2]),
                Technology::simple2(1, "t1", vec![], vec![2]),
                Technology::simple2(2, "t2", vec![0, 1], vec![3, 4]),
                Technology::simple2(3, "t3", vec![2], vec![]),
                Technology::simple2(4, "t4", vec![2], vec![]),
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
                Technology::simple2(0, "t0", vec![1], vec![]),
                Technology::simple2(1, "t1", vec![], vec![0]),
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
