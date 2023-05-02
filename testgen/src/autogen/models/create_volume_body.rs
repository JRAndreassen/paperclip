#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]
/*
 * IoEngine RESTful API
 *
 * The version of the OpenAPI document: v0
 *
 * Generated by: https://github.com/openebs/openapi-generator
 */

use crate::apis::{IntoOptVec, IntoVec};




/// CreateVolumeBody : Create Volume Body









/// Create Volume Body
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct CreateVolumeBody {

    
    /// Volume policy used to determine if and how to replace a replica
    #[serde(rename = "policy")]
    pub policy: crate::models::VolumePolicy,

    
    /// number of storage replicas
    #[serde(rename = "replicas")]
    pub replicas: u8,

    
    /// size of the volume in bytes
    #[serde(rename = "size")]
    pub size: u64,

    
    /// flag indicating whether or not the volume is thin provisioned
    #[serde(rename = "thin")]
    pub thin: bool,

    
    /// node and pool topology for volumes
    #[serde(rename = "topology", skip_serializing_if = "Option::is_none")]
    pub topology: Option<crate::models::Topology>,

    
    /// Optionally used to store custom volume information
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,

    
    /// Affinity Group related information.
    #[serde(rename = "affinity_group", skip_serializing_if = "Option::is_none")]
    pub affinity_group: Option<crate::models::AffinityGroup>,

}

impl CreateVolumeBody {
    /// CreateVolumeBody using only the required fields
    pub fn new(policy: impl Into<crate::models::VolumePolicy>, replicas: impl Into<u8>, size: impl Into<u64>, thin: impl Into<bool>) -> CreateVolumeBody {
        CreateVolumeBody {
            
            policy: policy.into(),
            
            replicas: replicas.into(),
            
            size: size.into(),
            
            thin: thin.into(),
            
            topology: None,
            
            labels: None,
            
            affinity_group: None,
            
        }
    }
    /// CreateVolumeBody using all fields
    pub fn new_all(policy: impl Into<crate::models::VolumePolicy>, replicas: impl Into<u8>, size: impl Into<u64>, thin: impl Into<bool>, topology: impl Into<Option<crate::models::Topology>>, labels: impl Into<Option<::std::collections::HashMap<String, String>>>, affinity_group: impl Into<Option<crate::models::AffinityGroup>>) -> CreateVolumeBody {
        CreateVolumeBody {
            
            policy: policy.into(),
            
            replicas: replicas.into(),
            
            size: size.into(),
            
            thin: thin.into(),
            
            topology: topology.into(),
            
            labels: labels.into(),
            
            affinity_group: affinity_group.into(),
            
        }
    }
}






















