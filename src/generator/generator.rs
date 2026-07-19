use crate::generator::point::Point;
use tokio::sync::broadcast::Sender;

#[async_trait::async_trait]
pub trait Generator {
    async fn generate_data(&mut self, prod: Sender<Point>);
}
