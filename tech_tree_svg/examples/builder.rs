use tech_tree::rendering::renderer::Renderer;
use tech_tree_svg::SvgBuilder;

fn main() {
    let mut builder = SvgBuilder::new(100, 150, 10, 10);

    builder.render_technology("Tech 1", 50, 20);
    builder.render_technology("Tech 2", 50, 70);
    builder.render_arrow(vec![(50, 30), (50, 60)]);
    builder.export("builder.svg");
}
