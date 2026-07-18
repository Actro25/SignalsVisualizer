use crate::generator::point::Point;
use crate::generator::signaler::Signaler;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::broadcast::Sender;
use tokio::sync::broadcast::error::SendError;

pub struct Signals {
    pub is_working: Arc<AtomicBool>,
}

impl Signals {
    pub fn new(state: Arc<AtomicBool>) -> Signals {
        Signals { is_working: state }
    }
}

#[async_trait::async_trait]
impl Signaler for Signals {
    async fn generate_data(&mut self, prod: Sender<Point>) {
        let mut start_x: f64 = 0.0;
        let dx: f64 = 0.005;
        while self.is_working.load(Ordering::Relaxed) {
            let start_y = start_x.sin();
            match prod.send(Point {
                x: start_x,
                y: start_y,
            }) {
                Err(e) => println!("There is no receivers currently."),
                _ => {}
            }
            start_x += dx;
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    }
}
