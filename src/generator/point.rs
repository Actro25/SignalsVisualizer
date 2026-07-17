use serde::Serialize;

#[derive(Serialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point {
            x: self.x,
            y: self.y,
        }
    }
}
