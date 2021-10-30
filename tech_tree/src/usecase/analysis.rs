use crate::model::technology::tree::TechnologyTree;
use crate::model::technology::TechnologyId;
use std::collections::VecDeque;

pub fn calculate_number_of_predecessors(tree: &TechnologyTree) -> Vec<u32> {
    let mut numbers = vec![0; tree.technologies().len()];
    let mut queue = VecDeque::from(calculate_technologies_without_predecessors(tree));

    while let Some(id) = queue.remove(0) {
        if let Some(technology) = tree.get(id) {
            let next: u32 = numbers[id.id()] + 1u32;

            for successor in technology.successors() {
                numbers[successor.id()] = next.max(numbers[successor.id()]);
                queue.push_back(*successor);
            }
        }
    }

    numbers
}

pub fn calculate_technologies_without_predecessors(tree: &TechnologyTree) -> Vec<TechnologyId> {
    tree.technologies()
        .iter()
        .filter(|t| t.predecessors().is_empty())
        .map(|t| *t.id())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::technology::Input;
    use crate::usecase::creation::create_tree;

    #[test]
    fn test_calculate_number_of_predecessors() {
        assert_eq!(
            calculate_number_of_predecessors(&init_tree()),
            vec![0, 0, 1, 2, 3]
        );
    }

    #[test]
    fn test_calculate_technologies_without_predecessors() {
        assert_eq!(
            calculate_technologies_without_predecessors(&init_tree()),
            vec![TechnologyId::new(0), TechnologyId::new(1),]
        );
    }

    fn init_tree() -> TechnologyTree {
        create_tree(vec![
            Input::test("t0", vec![]),
            Input::test("t1", vec![]),
            Input::test("t2", vec!["t0"]),
            Input::test("t3", vec!["t2", "t1"]),
            Input::test("t4", vec!["t3"]),
        ])
        .unwrap()
    }
}
