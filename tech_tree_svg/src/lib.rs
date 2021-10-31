use anyhow::{Context, Result};
use std::io::BufWriter;
use svg::node::element::path::Data;
use svg::node::element::Text;
use svg::node::element::{Definitions, Marker, Path, Rectangle};
use svg::{Document, Node};
use tech_tree::rendering::renderer::Renderer;

pub struct SvgBuilder {
    document: Document,
    font_size: u32,
    text_padding: u32,
}

impl SvgBuilder {
    pub fn new(font_size: u32, text_padding: u32) -> Self {
        Self {
            document: Document::new().add(Self::create_definitions()),
            font_size,
            text_padding,
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
        text.len() as u32 * font_width + 2 * self.text_padding
    }

    fn get_text_height(&self) -> u32 {
        self.font_size * 2
    }

    pub fn export(&self, path: &str) -> Result<()> {
        svg::save(path, &self.document).context(format!("Failed to export to {:?}", path))
    }

    pub fn export_as_string(&self) -> Result<String> {
        let mut buf = BufWriter::new(Vec::new());

        svg::write(&mut buf, &self.document).context("Failed to write the document")?;

        let bytes = buf.into_inner().context("Failed to get bytes")?;
        String::from_utf8(bytes).context("Failed to parse bytes")
    }
}

impl Renderer for SvgBuilder {
    fn init(&mut self, width: u32, height: u32) {
        self.document.assign("viewBox", (0, 0, width, height));
    }

    fn render_link(&mut self, points: Vec<(i32, i32)>) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_size_of_technology() {
        let builder = SvgBuilder::new(10, 20);

        assert_eq!(builder.get_size_of_technology("test"), (60, 20));
        assert_eq!(builder.get_size_of_technology("another"), (75, 20));
    }

    #[test]
    fn test_export() {
        let mut builder = SvgBuilder::new(10, 10);

        builder.init(100, 150);
        builder.render_technology("Tech 1", 50, 20);
        builder.render_technology("Tech 2", 50, 70);
        builder.render_link(vec![(50, 30), (50, 60)]);

        let result = "<svg viewBox=\"0 0 100 150\" xmlns=\"http://www.w3.org/2000/svg\">
<defs>
<marker id=\"head\" orient=\"auto\" refX=\"1\" refY=\"7\" viewBox=\"0 0 10 10\">
<path d=\"M0,0 L10,7 L0,14 z\" fill=\"black\"/>
</marker>
</defs>
<rect fill=\"#4fc3ff\" height=\"20\" stroke=\"black\" stroke-width=\"1\" width=\"50\" x=\"25\" y=\"10\"/>
<text font-size=\"10\" text-anchor=\"middle\" x=\"50\" y=\"23\">
Tech 1
</text>
<rect fill=\"#4fc3ff\" height=\"20\" stroke=\"black\" stroke-width=\"1\" width=\"50\" x=\"25\" y=\"60\"/>
<text font-size=\"10\" text-anchor=\"middle\" x=\"50\" y=\"73\">
Tech 2
</text>
<path d=\"M50,30 L50,60\" fill=\"none\" marker-end=\"url(#head)\" stroke=\"black\" stroke-width=\"1\"/>
</svg>";

        assert_eq!(builder.export_as_string().unwrap(), result.to_string());
    }
}
