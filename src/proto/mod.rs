pub mod placement_driver {
    tonic::include_proto!("placement_driver");
    pub use placement_driver_server::{PlacementDriver, PlacementDriverServer};
    pub use placement_driver_client::PlacementDriverClient;
}