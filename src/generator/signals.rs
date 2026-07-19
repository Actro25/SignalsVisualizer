use crate::generator::generator::Generator;
use crate::generator::point::Point;
use atomic_float::AtomicF64;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::broadcast::Sender;

pub struct Signals {
    pub is_working: Arc<AtomicBool>,
    pub amplitude: Arc<AtomicF64>,
    pub frequency: Arc<AtomicF64>,
}

impl Signals {
    pub fn new(state: Arc<AtomicBool>, ampl: Arc<AtomicF64>, freq: Arc<AtomicF64>) -> Signals {
        Signals {
            is_working: state,
            amplitude: ampl,
            frequency: freq,
        }
    }
}

#[async_trait::async_trait]
impl Generator for Signals {
    async fn generate_data(&mut self, prod: Sender<Point>) {
        let mut start_x: f64 = 0.0;
        while self.is_working.load(Ordering::Relaxed) {
            let start_y = self.amplitude.load(Ordering::Relaxed) * start_x.sin();
            match prod.send(Point {
                x: start_x,
                y: start_y,
            }) {
                Err(_) => println!("There is no receivers currently."),
                _ => {}
            }
            start_x += self.frequency.load(Ordering::Relaxed);
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    }
}
