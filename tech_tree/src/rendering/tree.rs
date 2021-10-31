use crate::model::technology::tree::TechnologyTree;
use crate::model::technology::TechnologyId;
use crate::rendering::grid::{Grid, GridCell};
use crate::rendering::renderer::Renderer;
use crate::usecase::analysis::{calculate_depth, group_by_depth};
use itertools::izip;

pub struct TreeRenderer {
    padding: u32,
}

impl TreeRenderer {
    pub fn new(padding: u32) -> Self {
        Self { padding }
    }

    pub fn render(&mut self, renderer: &mut dyn Renderer, tree: &TechnologyTree) {
        let grid = self.calculate_grid(renderer, tree);

        renderer.init(grid.width(), grid.height());

        for cell in grid.cells() {
            let technology = tree.get(cell.id).unwrap();

            renderer.render_technology(technology.name().get_full(), cell.center_x, cell.center_y);

            if !technology.successors().is_empty() {
                let link_start = cell.get_link_start();

                for successor in technology.successors() {
                    let successor_cell = grid.get_cell(*successor).unwrap();
                    let link_end = successor_cell.get_link_end();

                    renderer.render_link(vec![link_start, link_end]);
                }
            }
        }
    }

    fn calculate_grid(&self, renderer: &mut dyn Renderer, tree: &TechnologyTree) -> Grid {
        let depth = calculate_depth(tree);
        let groups = group_by_depth(&depth);
        let sizes = self.get_sizes(renderer, tree, &groups);
        let mut cells = Vec::new();

        let mut max_width = 0;
        let mut y = 0;

        for (column_id, column_size) in izip!(groups, sizes) {
            let mut x = 0;
            let mut column_height = 0;

            for (id, (width, height)) in izip!(column_id, column_size) {
                let padded_width = width + 2 * self.padding;
                let padded_height = height + 2 * self.padding;

                cells.push(GridCell::new(
                    id,
                    x + padded_width / 2,
                    y + padded_height / 2,
                    width / 2,
                    height / 2,
                ));

                x += padded_width;
                column_height = padded_height;
            }

            max_width = max_width.max(x);
            y += column_height;
        }

        Grid::new(max_width, y, cells)
    }

    fn get_sizes(
        &self,
        renderer: &mut dyn Renderer,
        tree: &TechnologyTree,
        groups: &[Vec<TechnologyId>],
    ) -> Vec<Vec<(u32, u32)>> {
        let mut sizes = Vec::new();

        for column in groups {
            let mut widths = Vec::new();
            let mut max_height = 0;

            for id in column {
                let technology = tree.get(*id).unwrap();
                let (width, height) = renderer.get_size_of_technology(technology.name().get_full());

                widths.push(width);
                max_height = max_height.max(height);
            }

            sizes.push(
                widths
                    .into_iter()
                    .map(|width| (width, max_height))
                    .collect(),
            );
        }

        sizes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::technology::Input;
    use crate::usecase::creation::create_tree;
    use std::collections::HashMap;

    #[derive(Default)]
    struct MockRender {
        pub width: u32,
        pub height: u32,
        pub technologies: HashMap<String, (u32, u32)>,
    }

    impl Renderer for MockRender {
        fn init(&mut self, width: u32, height: u32) {
            self.width = width;
            self.height = height;
        }

        fn render_link(&mut self, points: Vec<(u32, u32)>) {}

        fn render_technology(&mut self, text: &str, x: u32, y: u32) {
            self.technologies.insert(text.to_string(), (x, y));
        }

        fn get_size_of_technology(&self, text: &str) -> (u32, u32) {
            let l = text.len() as u32;
            (l * 10, l * 20)
        }
    }

    #[test]
    fn test_render() {
        let tree = init_tree();
        let mut renderer = MockRender::default();
        let mut tree_renderer = TreeRenderer::new(5);

        tree_renderer.render(&mut renderer, &tree);

        assert_eq!(renderer.width, 70);
        assert_eq!(renderer.height, 100);
        assert_eq!(
            renderer.technologies,
            HashMap::from([
                ("a".to_string(), (10, 15)),
                ("bb".to_string(), (15, 55)),
                ("ccc".to_string(), (50, 65)),
            ])
        );
    }

    fn init_tree() -> TechnologyTree {
        create_tree(vec![
            Input::test("a", vec![]),
            Input::test("bb", vec!["a"]),
            Input::test("ccc", vec!["a"]),
        ])
        .unwrap()
    }
}
