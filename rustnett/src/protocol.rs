use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Represents a request from a validator to a miner
#[async_trait]
pub trait SubnetRequest: Sized + Send + Sync {
    type Error;
    
    /// Validate the request format
    fn validate(&self) -> Result<(), Self::Error>;
}

/// Represents a response from a miner to a validator
#[async_trait]
pub trait SubnetResponse: Sized + Send + Sync {
    type Error;
    
    /// Validate the response format
    fn validate(&self) -> Result<(), Self::Error>;
}

/// Core subnet protocol definition
#[async_trait]
pub trait SubnetProtocol: Send + Sync {
    type Request: SubnetRequest;
    type Response: SubnetResponse;
    type Error;

    /// Process a request and generate a response
    async fn process_request(
        &self,
        request: Self::Request,
    ) -> Result<Self::Response, Self::Error>;

    /// Verify a response
    async fn verify_response(
        &self,
        request: &Self::Request,
        response: &Self::Response,
    ) -> Result<u16, Self::Error>;
}