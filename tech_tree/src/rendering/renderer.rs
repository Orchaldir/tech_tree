pub trait Renderer {
    /// Initializes the render target.
    fn init(&mut self, width: u32, height: u32);

    /// Renders a link between 2 technologies.
    fn render_link(&mut self, points: Vec<(u32, u32)>);

    /// Renders a technology.
    fn render_technology(&mut self, text: &str, x: u32, y: u32);

    /// Returns the size of a technology. Needed for calculating the layout.
    fn get_size_of_technology(&self, text: &str) -> (u32, u32);
}
