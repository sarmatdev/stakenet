use std::{collections::HashMap, sync::Arc};

use solana_client::nonblocking::rpc_client::RpcClient;
use tokio::sync::Mutex;
use validator_history::ValidatorHistory;

pub type AppState = Arc<Mutex<StateInternal>>;

pub struct StateInternal {
    pub rpc_client: RpcClient,
    pub cache: Cache,
}

impl StateInternal {
    pub fn new(rpc_client: RpcClient) -> Self {
        StateInternal {
            rpc_client,
            cache: Cache {
                validator_history_map: HashMap::new(),
            },
        }
    }
}

pub struct Cache {
    pub validator_history_map: HashMap<String, ValidatorHistory>,
}
