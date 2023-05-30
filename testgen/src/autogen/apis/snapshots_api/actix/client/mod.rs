#![allow(clippy::vec_init_then_push)]

use crate::clients::actix::{
    configuration, Error, ResponseContent, ResponseContentUnexpected,
};
use actix_web_opentelemetry::ClientExt;
use std::rc::Rc;

#[derive(Clone)]
pub struct SnapshotsClient {
    configuration: Rc<configuration::Configuration>,
}

impl SnapshotsClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> Self {
        Self {
            configuration,
        }
    }
}

#[async_trait::async_trait(?Send)]
#[dyn_clonable::clonable]
pub trait Snapshots: Clone {
    
    
    
    
    async fn get_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<crate::models::VolumeSnapshot, Error<crate::models::RestJsonError>>;
    
    
    
    async fn put_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<crate::models::VolumeSnapshot, Error<crate::models::RestJsonError>>;
    
    
    
    async fn del_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<(), Error<crate::models::RestJsonError>>;
    
    
    
    async fn get_volume_snapshots(&self, volume_id: &uuid::Uuid) -> Result<Vec<crate::models::VolumeSnapshot>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn del_snapshot(&self, snapshot_id: &uuid::Uuid) -> Result<(), Error<crate::models::RestJsonError>>;
    
    
    
    async fn get_snapshots(&self, ) -> Result<Vec<crate::models::VolumeSnapshot>, Error<crate::models::RestJsonError>>;
    
    
}

#[async_trait::async_trait(?Send)]
impl Snapshots for SnapshotsClient {
    
    
    
    
    async fn get_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<crate::models::VolumeSnapshot, Error<crate::models::RestJsonError>> {
    

        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!("{}/volumes/{volume_id}/snapshots/{snapshot_id}", configuration.base_path, volume_id=volume_id.to_string(), snapshot_id=snapshot_id.to_string());
        let mut local_var_req_builder = local_var_client.request(awc::http::Method::GET, local_var_uri_str.as_str());

        
        
        
        
        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder.insert_header((awc::http::header::USER_AGENT, local_var_user_agent.clone()));
        }
        
        
        
        
        
        
        
        let mut local_var_resp = if configuration.trace_requests {
            local_var_req_builder.trace_request().send().await
        } else {
            local_var_req_builder.send().await
        }?;
        

        let local_var_status = local_var_resp.status();
        
        if local_var_status.is_success() {
            
            
            
            let local_var_content = local_var_resp.json::<crate::models::VolumeSnapshot>().await?;
            Ok(local_var_content)
            
            
            
        } else {
            match local_var_resp.json::<crate::models::RestJsonError>().await {
                Ok(error) => Err(Error::ResponseError(ResponseContent {
                    status: local_var_status,
                    error,
                })),
                Err(_) => Err(Error::ResponseUnexpected(ResponseContentUnexpected {
                    status: local_var_status,
                    text: local_var_resp.json().await?,
                })),
            }
        }
    }
    
    
    
    async fn put_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<crate::models::VolumeSnapshot, Error<crate::models::RestJsonError>> {
    

        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!("{}/volumes/{volume_id}/snapshots/{snapshot_id}", configuration.base_path, volume_id=volume_id.to_string(), snapshot_id=snapshot_id.to_string());
        let mut local_var_req_builder = local_var_client.request(awc::http::Method::PUT, local_var_uri_str.as_str());

        
        
        
        
        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder.insert_header((awc::http::header::USER_AGENT, local_var_user_agent.clone()));
        }
        
        
        
        
        
        
        
        let mut local_var_resp = if configuration.trace_requests {
            local_var_req_builder.trace_request().send().await
        } else {
            local_var_req_builder.send().await
        }?;
        

        let local_var_status = local_var_resp.status();
        
        if local_var_status.is_success() {
            
            
            
            let local_var_content = local_var_resp.json::<crate::models::VolumeSnapshot>().await?;
            Ok(local_var_content)
            
            
            
        } else {
            match local_var_resp.json::<crate::models::RestJsonError>().await {
                Ok(error) => Err(Error::ResponseError(ResponseContent {
                    status: local_var_status,
                    error,
                })),
                Err(_) => Err(Error::ResponseUnexpected(ResponseContentUnexpected {
                    status: local_var_status,
                    text: local_var_resp.json().await?,
                })),
            }
        }
    }
    
    
    
    async fn del_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<(), Error<crate::models::RestJsonError>> {
    

        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!("{}/volumes/{volume_id}/snapshots/{snapshot_id}", configuration.base_path, volume_id=volume_id.to_string(), snapshot_id=snapshot_id.to_string());
        let mut local_var_req_builder = local_var_client.request(awc::http::Method::DELETE, local_var_uri_str.as_str());

        
        
        
        
        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder.insert_header((awc::http::header::USER_AGENT, local_var_user_agent.clone()));
        }
        
        
        
        
        
        
        
        let mut local_var_resp = if configuration.trace_requests {
            local_var_req_builder.trace_request().send().await
        } else {
            local_var_req_builder.send().await
        }?;
        

        let local_var_status = local_var_resp.status();
        
        if local_var_status.is_success() {
            
            
            
            let local_var_content = local_var_resp.json::<()>().await?;
            Ok(local_var_content)
            
            
            
        } else {
            match local_var_resp.json::<crate::models::RestJsonError>().await {
                Ok(error) => Err(Error::ResponseError(ResponseContent {
                    status: local_var_status,
                    error,
                })),
                Err(_) => Err(Error::ResponseUnexpected(ResponseContentUnexpected {
                    status: local_var_status,
                    text: local_var_resp.json().await?,
                })),
            }
        }
    }
    
    
    
    async fn get_volume_snapshots(&self, volume_id: &uuid::Uuid) -> Result<Vec<crate::models::VolumeSnapshot>, Error<crate::models::RestJsonError>> {
    

        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!("{}/volumes/{volume_id}/snapshots", configuration.base_path, volume_id=volume_id.to_string());
        let mut local_var_req_builder = local_var_client.request(awc::http::Method::GET, local_var_uri_str.as_str());

        
        
        
        
        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder.insert_header((awc::http::header::USER_AGENT, local_var_user_agent.clone()));
        }
        
        
        
        
        
        
        
        let mut local_var_resp = if configuration.trace_requests {
            local_var_req_builder.trace_request().send().await
        } else {
            local_var_req_builder.send().await
        }?;
        

        let local_var_status = local_var_resp.status();
        
        if local_var_status.is_success() {
            
            
            
            let local_var_content = local_var_resp.json::<Vec<crate::models::VolumeSnapshot>>().await?;
            Ok(local_var_content)
            
            
            
        } else {
            match local_var_resp.json::<crate::models::RestJsonError>().await {
                Ok(error) => Err(Error::ResponseError(ResponseContent {
                    status: local_var_status,
                    error,
                })),
                Err(_) => Err(Error::ResponseUnexpected(ResponseContentUnexpected {
                    status: local_var_status,
                    text: local_var_resp.json().await?,
                })),
            }
        }
    }
    
    
    
    async fn del_snapshot(&self, snapshot_id: &uuid::Uuid) -> Result<(), Error<crate::models::RestJsonError>> {
    

        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!("{}/snapshots/{snapshot_id}", configuration.base_path, snapshot_id=snapshot_id.to_string());
        let mut local_var_req_builder = local_var_client.request(awc::http::Method::DELETE, local_var_uri_str.as_str());

        
        
        
        
        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder.insert_header((awc::http::header::USER_AGENT, local_var_user_agent.clone()));
        }
        
        
        
        
        
        
        
        let mut local_var_resp = if configuration.trace_requests {
            local_var_req_builder.trace_request().send().await
        } else {
            local_var_req_builder.send().await
        }?;
        

        let local_var_status = local_var_resp.status();
        
        if local_var_status.is_success() {
            
            
            
            let local_var_content = local_var_resp.json::<()>().await?;
            Ok(local_var_content)
            
            
            
        } else {
            match local_var_resp.json::<crate::models::RestJsonError>().await {
                Ok(error) => Err(Error::ResponseError(ResponseContent {
                    status: local_var_status,
                    error,
                })),
                Err(_) => Err(Error::ResponseUnexpected(ResponseContentUnexpected {
                    status: local_var_status,
                    text: local_var_resp.json().await?,
                })),
            }
        }
    }
    
    
    
    async fn get_snapshots(&self, ) -> Result<Vec<crate::models::VolumeSnapshot>, Error<crate::models::RestJsonError>> {
    

        let configuration = &self.configuration;
        let local_var_client = &configuration.client;

        let local_var_uri_str = format!("{}/snapshots", configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(awc::http::Method::GET, local_var_uri_str.as_str());

        
        
        
        
        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder.insert_header((awc::http::header::USER_AGENT, local_var_user_agent.clone()));
        }
        
        
        
        
        
        
        
        let mut local_var_resp = if configuration.trace_requests {
            local_var_req_builder.trace_request().send().await
        } else {
            local_var_req_builder.send().await
        }?;
        

        let local_var_status = local_var_resp.status();
        
        if local_var_status.is_success() {
            
            
            
            let local_var_content = local_var_resp.json::<Vec<crate::models::VolumeSnapshot>>().await?;
            Ok(local_var_content)
            
            
            
        } else {
            match local_var_resp.json::<crate::models::RestJsonError>().await {
                Ok(error) => Err(Error::ResponseError(ResponseContent {
                    status: local_var_status,
                    error,
                })),
                Err(_) => Err(Error::ResponseUnexpected(ResponseContentUnexpected {
                    status: local_var_status,
                    text: local_var_resp.json().await?,
                })),
            }
        }
    }
    
    
}