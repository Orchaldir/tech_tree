use crate::model::error::AddError;
use crate::model::technology::tree::TechnologyTree;
use crate::model::technology::TechnologyId;

pub fn validate_no_cycles(tree: TechnologyTree) -> Result<TechnologyTree, AddError> {
    if let Some(circle) = validate_tree(&tree) {
        return Err(AddError::Cycle(circle));
    }

    Ok(tree)
}

fn validate_tree(tree: &TechnologyTree) -> Option<Vec<String>> {
    let len = tree.technologies().len();
    let mut visited = vec![false; len];
    let mut recursive = vec![false; len];

    for technology in tree.technologies() {
        if let Some(circle) =
            validate_technology(tree, *technology.id(), &mut visited, &mut recursive)
        {
            return Some(circle);
        }
    }

    None
}

fn validate_technology(
    tree: &TechnologyTree,
    id: TechnologyId,
    visited: &mut [bool],
    recursive: &mut [bool],
) -> Option<Vec<String>> {
    let i = id.id();

    if recursive[i] {
        return Some(get_circle(tree, id, recursive));
    } else if visited[i] {
        return None;
    }

    recursive[i] = true;
    visited[i] = true;

    for successor_id in tree.get(id).unwrap().successors() {
        if let Some(circle) = validate_technology(tree, *successor_id, visited, recursive) {
            return Some(circle);
        }
    }

    recursive[i] = false;

    None
}

fn get_circle(tree: &TechnologyTree, id: TechnologyId, recursive: &mut [bool]) -> Vec<String> {
    let mut circle = Vec::new();
    let mut current_id = Some(id);

    while let Some(id) = current_id {
        recursive[id.id()] = false;
        current_id = None;

        let technology = tree.get(id).unwrap();
        circle.push(technology.name().get_full().to_string());

        for successor_id in technology.successors() {
            if recursive[successor_id.id()] {
                current_id = Some(*successor_id);
                break;
            }
        }
    }

    circle
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::technology::Input;
    use crate::usecase::creation::create_tree;

    #[test]
    fn test_empty() {
        let tree = TechnologyTree::new(vec![]);

        assert_eq!(validate_tree(&tree), None);
    }

    #[test]
    fn test_no_cycle() {
        let tree = init_tree(vec![]);

        assert_eq!(validate_tree(&tree), None);
    }

    #[test]
    fn test_error() {
        let tree = init_tree(vec!["t3"]);

        assert_eq!(
            validate_no_cycles(tree),
            Err(AddError::Cycle(vec![
                "t0".to_string(),
                "t2".to_string(),
                "t3".to_string()
            ]))
        );
    }

    fn init_tree(predecessors0: Vec<&str>) -> TechnologyTree {
        create_tree(vec![
            Input::test("t0", predecessors0),
            Input::test("t1", vec![]),
            Input::test("t2", vec!["t0"]),
            Input::test("t3", vec!["t2", "t1"]),
            Input::test("t4", vec!["t3"]),
        ])
        .unwrap()
    }
}
