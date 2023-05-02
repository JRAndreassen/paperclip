#![allow(clippy::too_many_arguments, clippy::new_without_default, non_camel_case_types, unused_imports)]
/*
 * IoEngine RESTful API
 *
 * The version of the OpenAPI document: v0
 *
 * Generated by: https://github.com/openebs/openapi-generator
 */

use crate::apis::{IntoOptVec, IntoVec};




/// BlockDevice : Block device information









/// Block device information
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct BlockDevice {

    
    /// identifies if device is available for use (ie. is not \"currently\" in  use)
    #[serde(rename = "available")]
    pub available: bool,

    
    /// list of udev generated symlinks by which device may be identified
    #[serde(rename = "devlinks")]
    pub devlinks: Vec<String>,

    
    /// major device number
    #[serde(rename = "devmajor")]
    pub devmajor: i32,

    
    /// minor device number
    #[serde(rename = "devminor")]
    pub devminor: i32,

    
    /// entry in /dev associated with device
    #[serde(rename = "devname")]
    pub devname: String,

    
    /// official device path
    #[serde(rename = "devpath")]
    pub devpath: String,

    
    /// currently \"disk\" or \"partition\"
    #[serde(rename = "devtype")]
    pub devtype: String,

    
    /// filesystem information in case where a filesystem is present
    #[serde(rename = "filesystem")]
    pub filesystem: crate::models::BlockDeviceFilesystem,

    
    /// device model - useful for identifying devices
    #[serde(rename = "model")]
    pub model: String,

    
    /// partition information in case where device represents a partition
    #[serde(rename = "partition")]
    pub partition: crate::models::BlockDevicePartition,

    
    /// size of device in (512 byte) blocks
    #[serde(rename = "size")]
    pub size: u64,

}

impl BlockDevice {
    /// BlockDevice using only the required fields
    pub fn new(available: impl Into<bool>, devlinks: impl IntoVec<String>, devmajor: impl Into<i32>, devminor: impl Into<i32>, devname: impl Into<String>, devpath: impl Into<String>, devtype: impl Into<String>, filesystem: impl Into<crate::models::BlockDeviceFilesystem>, model: impl Into<String>, partition: impl Into<crate::models::BlockDevicePartition>, size: impl Into<u64>) -> BlockDevice {
        BlockDevice {
            
            available: available.into(),
            
            devlinks: devlinks.into_vec(),
            
            devmajor: devmajor.into(),
            
            devminor: devminor.into(),
            
            devname: devname.into(),
            
            devpath: devpath.into(),
            
            devtype: devtype.into(),
            
            filesystem: filesystem.into(),
            
            model: model.into(),
            
            partition: partition.into(),
            
            size: size.into(),
            
        }
    }
    /// BlockDevice using all fields
    pub fn new_all(available: impl Into<bool>, devlinks: impl IntoVec<String>, devmajor: impl Into<i32>, devminor: impl Into<i32>, devname: impl Into<String>, devpath: impl Into<String>, devtype: impl Into<String>, filesystem: impl Into<crate::models::BlockDeviceFilesystem>, model: impl Into<String>, partition: impl Into<crate::models::BlockDevicePartition>, size: impl Into<u64>) -> BlockDevice {
        BlockDevice {
            
            available: available.into(),
            
            devlinks: devlinks.into_vec(),
            
            devmajor: devmajor.into(),
            
            devminor: devminor.into(),
            
            devname: devname.into(),
            
            devpath: devpath.into(),
            
            devtype: devtype.into(),
            
            filesystem: filesystem.into(),
            
            model: model.into(),
            
            partition: partition.into(),
            
            size: size.into(),
            
        }
    }
}






























