//! Bindings for the S3N Squaring contracts
use alloy::sol;
use serde::{Deserialize, Serialize};

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    S3NTaskManager,
    "S3NTaskManager.json"
);

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    #[derive(Debug, Serialize, Deserialize)]
    S3NServiceManager,
    "S3NServiceManager.json"
);
