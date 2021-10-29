use svg::node::element::path::Data;
use svg::node::element::Text;
use svg::node::element::{Definitions, Marker, Path, Rectangle};
use svg::Document;
use tech_tree::rendering::renderer::Renderer;
use tech_tree_svg::SvgBuilder;

#[test]
fn test_document() {
    let x = 35i32;
    let y = 35i32;
    let font_size = 10i32;
    let font_width = font_size / 2;
    let padding = font_size;
    let text = "Hello";
    let text_offset = font_size / 3;
    let width = text.len() as i32 * font_width + 2 * padding;
    let width_half = width / 2;
    let height = font_size * 2;

    let arrow_head_data = Data::new()
        .move_to((0, 0))
        .line_to((10, 7))
        .line_to((0, 14))
        .close();

    let arrow_head_path = Path::new().set("fill", "black").set("d", arrow_head_data);

    let arrow_head = Marker::new()
        .set("id", "head")
        .set("viewBox", (0, 0, 10, 10))
        .set("orient", "auto")
        .set("refX", 1)
        .set("refY", 7)
        .add(arrow_head_path);

    let definitions = Definitions::new().add(arrow_head);

    let box_node = Rectangle::new()
        .set("x", x - width_half)
        .set("y", y - font_size)
        .set("width", width)
        .set("height", height)
        .set("fill", "#4fc3ff")
        .set("stroke", "black")
        .set("stroke-width", 1);

    let text_element = svg::node::Text::new(text);

    let text_node = Text::new()
        .set("x", x)
        .set("y", y + text_offset)
        .set("font-size", font_size)
        .set("text-anchor", "middle")
        //.set("dominant-baseline", "middle")
        .add(text_element);

    let arrow_data = Data::new()
        .move_to((10, 10))
        .line_by((32, 52))
        .line_by((10, -52));

    let arrow_path = Path::new()
        .set("marker-end", "url(#head)")
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("d", arrow_data);

    let document = Document::new()
        .set("viewBox", (0, 0, 70, 70))
        .add(definitions)
        .add(box_node)
        .add(text_node)
        .add(arrow_path);

    svg::save("document.svg", &document).unwrap();
}

#[test]
fn test_builder() {
    let mut builder = SvgBuilder::new(100, 150, 10, 10);

    builder.render_technology("Tech 1", 50, 20);
    builder.render_technology("Tech 2", 50, 70);
    builder.render_arrow(vec![(50, 30), (50, 60)]);
    builder.export("builder.svg");
}
