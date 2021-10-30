pub trait Renderer {
    fn init(&mut self, width: u32, height: u32);

    fn render_arrow(&mut self, points: Vec<(i32, i32)>);

    fn render_technology(&mut self, text: &str, x: u32, y: u32);

    fn get_size_of_technology(&self, text: &str) -> (u32, u32);
}
