pub struct Window {
    width: u32,
    height: u32,
    x_axis: i32,
    y_axis: i32,
}

impl Window {
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn new(width: u32, height: u32, x_axis: i32, y_axis: i32) -> Self {
        Self {
            width,
            height,
            x_axis,
            y_axis,
        }
    }
}
