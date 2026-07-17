use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::broadcast::Sender;
use crate::generator::point::Point;

pub struct Signals{
    pub is_working: Arc<AtomicBool>
}

impl Signals {
    pub fn new(state: Arc<AtomicBool>) -> Signals {
        Signals { is_working: state }
    }

    pub async fn generate_data(
        &mut self,
        prod: Sender<Point>,
        mut start_x: f64,
        dx: f64,
    ) {

        while self.is_working.load(Ordering::Relaxed) {
            let start_y = start_x.sin();
            println!("signal");
            let _ =prod.send(Point { x: start_x, y: start_y });

            start_x += dx;

            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    }
}