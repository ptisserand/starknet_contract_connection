use starknet::core::types::Felt;
use starknet::core::codec::Encode;
use serde::{Serialize, Deserialize};

pub type ContextId = Felt;

// Context Member ID
pub type ContextIdentity = Felt;

#[derive(Debug, Encode)]
pub struct Signed {
    pub payload: Vec<Felt>,
    pub signature: Vec<Felt>,
}

#[derive(Encode)]
pub struct Request {
    pub kind: RequestKind,
    pub signer_id: ContextIdentity,
    pub nonce: u64,
}

#[derive(Debug, Serialize, Deserialize, Encode)]
pub enum RequestKind {
    Context(ContextRequest),
}

#[derive(Debug, Serialize, Deserialize, Encode)]
pub struct ContextRequest {
    pub context_id: ContextId,
    pub kind: ContextRequestKind,
}

#[derive(Debug, Serialize, Deserialize, Encode)]
pub enum ContextRequestKind {
    Add(ContextIdentity, Application),
}

// Context Application
#[derive(Debug, Serialize, Deserialize, Clone, Encode)]
pub struct Application {
  pub id: Felt,  // Represents [u8; 32]
  pub blob: Felt,  // Represents [u8; 32]
  pub size: u64,
  pub source: Vec<u8>,  // Encoding-friendly representation
  pub metadata: Vec<u8>
}
