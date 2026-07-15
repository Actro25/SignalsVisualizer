use crate::handlers::index::Points;
use ringbuf::traits::Split;
use ringbuf::{HeapRb, Prod};

pub struct Point(pub f64,pub f64);

pub struct Signals {
    pub ring_buffer: HeapRb<Point>,
}

impl Signals {
    pub fn new(capacity: usize) -> Signals {
        Signals {
            ring_buffer: HeapRb::<Point>::new(capacity),
        }
    }

    pub async fn generate_data(
        mut prod: impl ringbuf::traits::Producer<Item = Point>,
        mut start_x: f64,
        dx: f64,
    ) {
        loop {

            let start_y = start_x.sin();

            let _ =prod.try_push(Point(start_y, start_x));

            start_x += dx;

            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    }
}