use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use atomic_float::AtomicF64;
use crate::generator::point::Point;
use tokio::sync::broadcast::Sender;
use crate::generator::signals::Signals;

#[async_trait::async_trait]
pub trait Generator {
    async fn generate_data(&mut self, prod: Sender<Point>);
    fn new(state: Arc<AtomicBool>, ampl: Arc<AtomicF64>, freq: Arc<AtomicF64>) -> Signals;
}
