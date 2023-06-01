use std::sync::{Arc, Mutex};
use tonic::Request;

use crate::error::{Result, RpcResult};
use crate::proto::placement_driver::{PlacementDriver, TsoRequest, TsoReply, DataLocRequest, DataLocReply};

/// A featherPD server with a TSO.
pub struct FeatherPD {
    /// The next timestamp to be assigned.
    next_ts: Arc<Mutex<u64>>,
}

impl FeatherPD {
    /// Creates a new FeatherPD server.
    pub fn new() -> Result<Self> {
        Ok(Self {
            next_ts: Arc::new(Mutex::new(0)),
        })
    }

    /// Gets the next timestamp.
    pub fn get_next_ts(&self) -> u64 {
        let mut next_ts = self.next_ts.lock().unwrap();
        *next_ts += 1;
        *next_ts
    }
}

#[tonic::async_trait]
impl PlacementDriver for FeatherPD {
    async fn get_timestamp(&self, _request: Request<TsoRequest>) -> RpcResult<TsoReply> {
        todo!()
    }

    async fn get_data_location(&self, _request: Request<DataLocRequest>) -> RpcResult<DataLocReply> {
        todo!()
    }
}