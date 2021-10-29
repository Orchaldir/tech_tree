pub trait Renderer {
    fn render_arrow(&mut self, points: Vec<(i32, i32)>);

    fn render_technology(&mut self, text: &str, x: u32, y: u32);
}
