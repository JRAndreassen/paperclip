#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]
/*
 * IoEngine RESTful API
 *
 * The version of the OpenAPI document: v0
 *
 * Generated by: https://github.com/openebs/openapi-generator
 */

use crate::apis::{IntoOptVec, IntoVec};




/// ReplicaSnapshotState : Replica Snapshot state information.









/// Replica Snapshot state information.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReplicaSnapshotState {

    
    #[serde(rename = "uuid")]
    pub uuid: uuid::Uuid,

    
    #[serde(rename = "source_id")]
    pub source_id: uuid::Uuid,

    
    /// Timestamp when replica snapshot is taken on the storage system.
    #[serde(rename = "creation_timestamp")]
    pub creation_timestamp: String,

    
    /// Replica snapshot size.
    #[serde(rename = "size")]
    pub size: i64,

    
    /// Replica snapshot referenced size.
    #[serde(rename = "referenced_size")]
    pub referenced_size: i64,

    
    /// Current ReplicaSnapshot status
    #[serde(rename = "state")]
    pub state: crate::models::ReplicaSnapshotStatus,

}

impl ReplicaSnapshotState {
    /// ReplicaSnapshotState using only the required fields
    pub fn new(uuid: impl Into<uuid::Uuid>, source_id: impl Into<uuid::Uuid>, creation_timestamp: impl Into<String>, size: impl Into<i64>, referenced_size: impl Into<i64>, state: impl Into<crate::models::ReplicaSnapshotStatus>) -> ReplicaSnapshotState {
        ReplicaSnapshotState {
            
            uuid: uuid.into(),
            
            source_id: source_id.into(),
            
            creation_timestamp: creation_timestamp.into(),
            
            size: size.into(),
            
            referenced_size: referenced_size.into(),
            
            state: state.into(),
            
        }
    }
    /// ReplicaSnapshotState using all fields
    pub fn new_all(uuid: impl Into<uuid::Uuid>, source_id: impl Into<uuid::Uuid>, creation_timestamp: impl Into<String>, size: impl Into<i64>, referenced_size: impl Into<i64>, state: impl Into<crate::models::ReplicaSnapshotStatus>) -> ReplicaSnapshotState {
        ReplicaSnapshotState {
            
            uuid: uuid.into(),
            
            source_id: source_id.into(),
            
            creation_timestamp: creation_timestamp.into(),
            
            size: size.into(),
            
            referenced_size: referenced_size.into(),
            
            state: state.into(),
            
        }
    }
}




















