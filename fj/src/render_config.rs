#[derive(Debug)]
pub struct RenderConfig {
    pub draw_model: bool,
    pub draw_mesh: bool,
    pub draw_grid: bool,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            draw_model: true,
            draw_mesh: false,
            draw_grid: false,
        }
    }
}
