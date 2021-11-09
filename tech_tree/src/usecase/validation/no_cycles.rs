use crate::model::technology::tree::TechnologyTree;
use crate::model::technology::TechnologyId;

pub fn validate_no_cycles(tree: &TechnologyTree) -> bool {
    let len = tree.technologies().len();
    let mut visited = vec![false; len];
    let mut recursive = vec![false; len];

    for technology in tree.technologies() {
        if validate(tree, *technology.id(), &mut visited, &mut recursive) {
            return true;
        }
    }

    false
}

fn validate(
    tree: &TechnologyTree,
    id: TechnologyId,
    visited: &mut [bool],
    recursive: &mut [bool],
) -> bool {
    let i = id.id();

    if recursive[i] {
        return true;
    } else if visited[i] {
        return false;
    }

    recursive[i] = true;
    visited[i] = true;

    for successor_id in tree.get(id).unwrap().successors() {
        if validate(tree, *successor_id, visited, recursive) {
            return true;
        }
    }

    recursive[i] = false;

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::technology::Input;
    use crate::usecase::creation::create_tree;

    #[test]
    fn test_empty() {
        let tree = TechnologyTree::new(vec![]);

        assert!(!validate_no_cycles(&tree));
    }

    #[test]
    fn test_no_cycle() {
        let tree = init_tree(vec![]);

        assert!(!validate_no_cycles(&tree));
    }

    #[test]
    fn test_cycle() {
        let tree = init_tree(vec!["t4"]);

        assert!(validate_no_cycles(&tree));
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
