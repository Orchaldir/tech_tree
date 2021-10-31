use tech_tree::model::technology::Input;
use tech_tree::rendering::tree::TreeRenderer;
use tech_tree::usecase::creation::create_tree;
use tech_tree_svg::SvgBuilder;

fn main() {
    let tree = create_tree(vec![
        Input::test("Technology 0", vec![]),
        Input::test("Technology 1", vec![]),
        Input::test(
            "Technology 2",
            vec!["Technology 0", "Technology 1", "Technology 5"],
        ),
        Input::test("Technology 3", vec!["Technology 2"]),
        Input::test(
            "Technology 4",
            vec!["Technology 2", "Technology 1", "Technology 5"],
        ),
        Input::test("Technology 5", vec![]),
    ])
    .unwrap();
    let mut builder = SvgBuilder::new(10, 10);
    let mut tree_renderer = TreeRenderer::new(20);

    tree_renderer.render(&mut builder, &tree);

    builder.export("tree.svg").expect("Failed test");
}
