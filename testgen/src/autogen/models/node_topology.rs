#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]
/*
 * IoEngine RESTful API
 *
 * The version of the OpenAPI document: v0
 *
 * Generated by: https://github.com/openebs/openapi-generator
 */

use crate::apis::{IntoOptVec, IntoVec};




/// NodeTopology : Used to determine how to place/distribute the data during volume creation and replica replacement.  If left empty then the control plane will select from all available resources.









/// Used to determine how to place/distribute the data during volume creation and replica replacement.  If left empty then the control plane will select from all available resources.


#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum NodeTopology {

    
    /// volume topology, explicitly selected
    #[serde(rename = "explicit")]
    explicit(crate::models::ExplicitNodeTopology),

    
    /// volume topology definition through labels
    #[serde(rename = "labelled")]
    labelled(crate::models::LabelledTopology),

}












