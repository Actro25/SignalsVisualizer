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

mod tests {
    use super::*;
    use tokio::sync::broadcast;

    #[tokio::test]
    async fn test_correct_amplitude() {
        let working = Arc::new(AtomicBool::new(true));
        let frequency = Arc::new(AtomicF64::from(0.1));
        let amplitude = Arc::new(AtomicF64::from(5.0));

        let mut signal = Signals::new(working.clone(), amplitude.clone(), frequency);

        let (producer, mut consumer) = broadcast::channel::<Point>(10);

        tokio::spawn(async move {
            signal.generate_data(producer).await;
        });

        for _ in 0..20 {
            if let Ok(point) = consumer.recv().await {
                assert!(
                    point.y <= amplitude.load(Ordering::Relaxed),
                    "There is an error, y is above {}.",
                    amplitude.load(Ordering::Relaxed)
                );
                assert!(
                    point.y >= -amplitude.load(Ordering::Relaxed),
                    "There is an error, y is below {}.",
                    amplitude.load(Ordering::Relaxed)
                );
            }
        }

        working.store(false, Ordering::Relaxed);
    }

    #[tokio::test]
    async fn test_correct_amplitude_changing_on_fly() {
        let working = Arc::new(AtomicBool::new(true));
        let frequency = Arc::new(AtomicF64::from(0.1));
        let amplitude = Arc::new(AtomicF64::from(1.0));

        let mut signal = Signals::new(working.clone(), amplitude.clone(), frequency);

        let (producer, mut consumer) = broadcast::channel::<Point>(10);

        tokio::spawn(async move {
            signal.generate_data(producer).await;
        });

        amplitude.store(10.0, Ordering::Relaxed);

        for _ in 0..20 {
            if let Ok(point) = consumer.recv().await {
                assert!(
                    point.y <= amplitude.load(Ordering::Relaxed),
                    "There is an error, y is above {}.",
                    amplitude.load(Ordering::Relaxed)
                );
                assert!(
                    point.y >= -amplitude.load(Ordering::Relaxed),
                    "There is an error, y is below {}.",
                    amplitude.load(Ordering::Relaxed)
                );
            }
        }

        working.store(false, Ordering::Relaxed);
    }

    #[tokio::test]
    async fn test_correct_frequency() {
        let working = Arc::new(AtomicBool::new(true));
        let frequency = Arc::new(AtomicF64::from(0.1));
        let amplitude = Arc::new(AtomicF64::from(1.0));

        let mut signal = Signals::new(working.clone(), amplitude.clone(), frequency.clone());

        let (producer, mut consumer) = broadcast::channel::<Point>(10);

        tokio::spawn(async move {
            signal.generate_data(producer).await;
        });

        let point1 = consumer.recv().await.unwrap();
        let point2 = consumer.recv().await.unwrap();

        let dx = (point1.x - point2.x).abs();

        assert!(
            dx - frequency.load(Ordering::Relaxed) <= 0.0001,
            "Here is an error, x: {} - freq: {} not <= 0.0001",
            dx,
            frequency.load(Ordering::Relaxed)
        );

        working.store(false, Ordering::Relaxed);
    }

    #[tokio::test]
    async fn test_correct_frequency_changing_on_fly() {
        let working = Arc::new(AtomicBool::new(true));
        let frequency = Arc::new(AtomicF64::from(0.1));
        let amplitude = Arc::new(AtomicF64::from(1.0));

        let mut signal = Signals::new(working.clone(), amplitude.clone(), frequency.clone());

        let (producer, mut consumer) = broadcast::channel::<Point>(10);

        tokio::spawn(async move {
            signal.generate_data(producer).await;
        });

        frequency.store(0.5, Ordering::Relaxed);

        let mut new_freq = consumer.recv().await.unwrap();

        for _ in 0..20 {
            if let Ok(point) = consumer.recv().await {
                let dx = (new_freq.x - point.x).abs();
                assert!(
                    dx - frequency.load(Ordering::Relaxed) <= 0.0001,
                    "Here is an error, x: {} - freq: {} not <= 0.0001",
                    dx,
                    frequency.load(Ordering::Relaxed)
                );
                new_freq = consumer.recv().await.unwrap();
            }
        }

        working.store(false, Ordering::Relaxed);
    }

    #[tokio::test]
    async fn test_correct_working() {
        let working = Arc::new(AtomicBool::new(true));
        let frequency = Arc::new(AtomicF64::from(0.1));
        let amplitude = Arc::new(AtomicF64::from(5.0));

        let mut signal = Signals::new(working.clone(), amplitude, frequency);

        let (producer, mut consumer) = broadcast::channel::<Point>(1);

        tokio::spawn(async move {
            signal.generate_data(producer).await;
        });

        working.store(false, Ordering::Relaxed);

        let _ = consumer.recv().await;

        if let Err(tokio::sync::broadcast::error::RecvError::Closed) = consumer.recv().await {
            assert!(true);
        } else {
            assert!(
                false,
                "Here is an error, generator aren't closed but he should. Working state: {}",
                working.load(Ordering::Relaxed)
            );
        }
    }
}
