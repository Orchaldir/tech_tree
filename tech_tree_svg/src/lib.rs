use svg::node::element::path::Data;
use svg::node::element::Text;
use svg::node::element::{Definitions, Marker, Path, Rectangle};
use svg::{Document, Node};
use tech_tree::rendering::renderer::Renderer;

pub struct SvgBuilder {
    document: Document,
    font_size: u32,
    padding: u32,
}

impl SvgBuilder {
    pub fn new(font_size: u32, padding: u32) -> Self {
        Self {
            document: Document::new().add(Self::create_definitions()),
            font_size,
            padding,
        }
    }

    fn create_definitions() -> Definitions {
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

        Definitions::new().add(arrow_head)
    }

    fn get_text_width(&self, text: &str) -> u32 {
        let font_width = self.font_size / 2;
        text.len() as u32 * font_width + 2 * self.padding
    }

    fn get_text_height(&self) -> u32 {
        self.font_size * 2
    }

    pub fn export(&self, path: &str) {
        svg::save(path, &self.document).unwrap();
    }
}

impl Renderer for SvgBuilder {
    fn init(&mut self, width: u32, height: u32) {
        self.document.assign("viewBox", (0, 0, width, height));
    }

    fn render_arrow(&mut self, points: Vec<(i32, i32)>) {
        if let Some((start, line)) = points.split_first() {
            let mut arrow_data = Data::new().move_to(*start);

            for point in line {
                arrow_data = arrow_data.line_to(*point);
            }

            let arrow_path = Path::new()
                .set("marker-end", "url(#head)")
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 1)
                .set("d", arrow_data);

            self.document.append(arrow_path);
        }
    }

    fn render_technology(&mut self, text: &str, x: u32, y: u32) {
        let text_offset = self.font_size / 3;
        let width = self.get_text_width(text);
        let width_half = width / 2;

        let box_node = Rectangle::new()
            .set("x", x - width_half)
            .set("y", y.saturating_sub(self.font_size))
            .set("width", width)
            .set("height", self.get_text_height())
            .set("fill", "#4fc3ff")
            .set("stroke", "black")
            .set("stroke-width", 1);

        let text_element = svg::node::Text::new(text);

        let text_node = Text::new()
            .set("x", x)
            .set("y", y + text_offset)
            .set("font-size", self.font_size)
            .set("text-anchor", "middle")
            //.set("dominant-baseline", "middle")
            .add(text_element);

        self.document.append(box_node);
        self.document.append(text_node);
    }

    fn get_size_of_technology(&self, text: &str) -> (u32, u32) {
        (self.get_text_width(text), self.get_text_height())
    }
}
