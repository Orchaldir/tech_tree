use svg::node::element::Rectangle;
use svg::node::element::Text;
use svg::Document;

#[test]
fn create_svg() {
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

    let document = Document::new()
        .set("viewBox", (0, 0, 70, 70))
        .add(box_node)
        .add(text_node);

    svg::save("image.svg", &document).unwrap();
}
