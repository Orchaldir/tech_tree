use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::node::element::Text;
use svg::Document;

#[test]
fn create_svg() {
    let data = Data::new()
        .move_to((10, 10))
        .line_by((0, 50))
        .line_by((50, 0))
        .line_by((0, -50))
        .close();

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("d", data);

    let element = svg::node::Text::new("Hello");

    let text = Text::new()
        .set("x", 35)
        .set("y", 35)
        .set("font-size", 10)
        .set("text-anchor", "middle")
        .set("dominant-baseline", "middle")
        .add(element);

    let document = Document::new()
        .set("viewBox", (0, 0, 70, 70))
        .add(path)
        .add(text);

    svg::save("image.svg", &document).unwrap();
}
