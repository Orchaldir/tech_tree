use crate::model::technology::tree::TechnologyTree;
use crate::model::technology::{Technology, TechnologyId};
use crate::rendering::renderer::Renderer;
use crate::usecase::analysis::{calculate_depth, group_by_depth};

pub struct TreeRenderer {
    padding: u32,
}

impl TreeRenderer {
    pub fn new(padding: u32) -> Self {
        Self { padding }
    }

    pub fn render(&mut self, renderer: &mut dyn Renderer, tree: &TechnologyTree) {
        let depth = calculate_depth(tree);
        let groups = group_by_depth(&depth);
        let (width, height) = self.get_size(renderer, &groups, tree);

        renderer.init(width, height);

        let mut y = 0;

        for column in groups {
            let mut x = 0;
            let mut max_height = 0;

            for id in column {
                let technology = tree.get(id).unwrap();
                let (t_width, t_height) = self.get_technology_size(renderer, technology);

                renderer.render_technology(
                    technology.name().get_full(),
                    x + t_width / 2,
                    y + t_height / 2,
                );

                x += t_width;
                max_height = max_height.max(t_height);
            }

            y += max_height;
        }
    }

    fn get_size(
        &self,
        renderer: &mut dyn Renderer,
        groups: &[Vec<TechnologyId>],
        tree: &TechnologyTree,
    ) -> (u32, u32) {
        let mut width = 0;
        let mut height = 0;

        for column in groups {
            let mut column_width = 0;
            let mut max_height = 0;

            for id in column {
                let technology = tree.get(*id).unwrap();
                let (t_width, t_height) = self.get_technology_size(renderer, technology);

                column_width += t_width;
                max_height = max_height.max(t_height);
            }

            width = width.max(column_width);
            height += max_height;
        }

        (width, height)
    }

    fn get_technology_size(
        &self,
        renderer: &mut dyn Renderer,
        technology: &Technology,
    ) -> (u32, u32) {
        let (width, height) = renderer.get_size_of_technology(technology.name().get_full());

        (width + 2 * self.padding, height + 2 * self.padding)
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

        fn render_link(&mut self, _points: Vec<(i32, i32)>) {}

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
