#![allow(clippy::to_string_in_format_args)]

use crate::clients::tower::{
    configuration, Error, RequestError, ResponseContent, ResponseContentUnexpected, ResponseError,
};

use hyper::service::Service;
use std::sync::Arc;
use tower::ServiceExt;

pub struct SnapshotsClient {
    configuration: Arc<configuration::Configuration>,
}

impl SnapshotsClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self {
            configuration,
        }
    }
}
impl Clone for SnapshotsClient {
    fn clone(&self) -> Self {
        Self {
            configuration: self.configuration.clone()
        }
    }
}

#[async_trait::async_trait]
#[dyn_clonable::clonable]
pub trait Snapshots: Clone + Send + Sync {
    
    
    
    
    async fn get_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<ResponseContent<crate::models::VolumeSnapshot>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn put_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<ResponseContent<crate::models::VolumeSnapshot>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn del_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn get_volume_snapshots(&self, volume_id: &uuid::Uuid) -> Result<ResponseContent<Vec<crate::models::VolumeSnapshot>>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn del_snapshot(&self, snapshot_id: &uuid::Uuid) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>>;
    
    
    
    async fn get_snapshots(&self, ) -> Result<ResponseContent<Vec<crate::models::VolumeSnapshot>>, Error<crate::models::RestJsonError>>;
    
    
}

/// Same as `Snapshots` but it returns the result body directly.
pub mod direct {
    #[async_trait::async_trait]
    #[dyn_clonable::clonable]
    pub trait Snapshots: Clone + Send + Sync {
        
        
        
        
        async fn get_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<crate::models::VolumeSnapshot, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn put_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<crate::models::VolumeSnapshot, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn del_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<(), super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn get_volume_snapshots(&self, volume_id: &uuid::Uuid) -> Result<Vec<crate::models::VolumeSnapshot>, super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn del_snapshot(&self, snapshot_id: &uuid::Uuid) -> Result<(), super::Error<crate::models::RestJsonError>>;
        
        
        
        async fn get_snapshots(&self, ) -> Result<Vec<crate::models::VolumeSnapshot>, super::Error<crate::models::RestJsonError>>;
        
        
    }
}

#[async_trait::async_trait]
impl direct::Snapshots for SnapshotsClient {
    
    
    
    
    async fn get_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<crate::models::VolumeSnapshot, Error<crate::models::RestJsonError>> {
    
        Snapshots::get_volume_snapshot(self, volume_id, snapshot_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn put_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<crate::models::VolumeSnapshot, Error<crate::models::RestJsonError>> {
    
        Snapshots::put_volume_snapshot(self, volume_id, snapshot_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn del_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<(), Error<crate::models::RestJsonError>> {
    
        Snapshots::del_volume_snapshot(self, volume_id, snapshot_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn get_volume_snapshots(&self, volume_id: &uuid::Uuid) -> Result<Vec<crate::models::VolumeSnapshot>, Error<crate::models::RestJsonError>> {
    
        Snapshots::get_volume_snapshots(self, volume_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn del_snapshot(&self, snapshot_id: &uuid::Uuid) -> Result<(), Error<crate::models::RestJsonError>> {
    
        Snapshots::del_snapshot(self, snapshot_id).await.map(|r| r.into_body())
    }
    
    
    
    async fn get_snapshots(&self, ) -> Result<Vec<crate::models::VolumeSnapshot>, Error<crate::models::RestJsonError>> {
    
        Snapshots::get_snapshots(self, ).await.map(|r| r.into_body())
    }
    
    
}

#[async_trait::async_trait]
impl Snapshots for SnapshotsClient {
    
    
    
    
    async fn get_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<ResponseContent<crate::models::VolumeSnapshot>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/volumes/{volume_id}/snapshots/{snapshot_id}", configuration.base_path, volume_id=volume_id.to_string(), snapshot_id=snapshot_id.to_string());
        let mut local_var_req_builder = hyper::Request::builder().method(hyper::Method::GET);

        
        
        
        
        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(hyper::header::USER_AGENT, local_var_user_agent.clone());
        }
        
        
        
        
        
        
        
        let body = hyper::Body::empty();
        
        let request = local_var_req_builder.uri(local_var_uri_str).header("content-type", "application/json").body(body).map_err(RequestError::BuildRequest)?;

        let local_var_resp = {
            let mut client_service = configuration.client_service.lock().await.clone();
            client_service
                .ready()
                .await
                .map_err(RequestError::NotReady)?
                .call(request)
                .await
                .map_err(RequestError::Request)?
        };
        let local_var_status = local_var_resp.status();
        
        if local_var_status.is_success() {
            
            
            
            let body = hyper::body::to_bytes(local_var_resp.into_body()).await.map_err(|e| ResponseError::PayloadError {
                status: local_var_status,
                error: e,
            })?;
            let local_var_content: crate::models::VolumeSnapshot =
                serde_json::from_slice(&body).map_err(|e| {
                    ResponseError::Unexpected(ResponseContentUnexpected {
                        status: local_var_status,
                        text: e.to_string(),
                    })
                })?;
            Ok(ResponseContent { status: local_var_status, body: local_var_content })
            
            
            
        } else {
            match hyper::body::to_bytes(local_var_resp.into_body()).await {
                Ok(body) => match serde_json::from_slice::<crate::models::RestJsonError>(&body) {
                    Ok(error) => Err(Error::Response(ResponseError::Expected(ResponseContent {
                        status: local_var_status,
                        body: error,
                    }))),
                    Err(_) => Err(Error::Response(ResponseError::Unexpected(
                        ResponseContentUnexpected {
                            status: local_var_status,
                            text: String::from_utf8_lossy(&body).to_string(),
                        },
                    ))),
                },
                Err(error) => Err(Error::Response(ResponseError::PayloadError {
                    status: local_var_status,
                    error,
                })),
            }
            
        }
    }
    
    
    
    async fn put_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<ResponseContent<crate::models::VolumeSnapshot>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/volumes/{volume_id}/snapshots/{snapshot_id}", configuration.base_path, volume_id=volume_id.to_string(), snapshot_id=snapshot_id.to_string());
        let mut local_var_req_builder = hyper::Request::builder().method(hyper::Method::PUT);

        
        
        
        
        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(hyper::header::USER_AGENT, local_var_user_agent.clone());
        }
        
        
        
        
        
        
        
        let body = hyper::Body::empty();
        
        let request = local_var_req_builder.uri(local_var_uri_str).header("content-type", "application/json").body(body).map_err(RequestError::BuildRequest)?;

        let local_var_resp = {
            let mut client_service = configuration.client_service.lock().await.clone();
            client_service
                .ready()
                .await
                .map_err(RequestError::NotReady)?
                .call(request)
                .await
                .map_err(RequestError::Request)?
        };
        let local_var_status = local_var_resp.status();
        
        if local_var_status.is_success() {
            
            
            
            let body = hyper::body::to_bytes(local_var_resp.into_body()).await.map_err(|e| ResponseError::PayloadError {
                status: local_var_status,
                error: e,
            })?;
            let local_var_content: crate::models::VolumeSnapshot =
                serde_json::from_slice(&body).map_err(|e| {
                    ResponseError::Unexpected(ResponseContentUnexpected {
                        status: local_var_status,
                        text: e.to_string(),
                    })
                })?;
            Ok(ResponseContent { status: local_var_status, body: local_var_content })
            
            
            
        } else {
            match hyper::body::to_bytes(local_var_resp.into_body()).await {
                Ok(body) => match serde_json::from_slice::<crate::models::RestJsonError>(&body) {
                    Ok(error) => Err(Error::Response(ResponseError::Expected(ResponseContent {
                        status: local_var_status,
                        body: error,
                    }))),
                    Err(_) => Err(Error::Response(ResponseError::Unexpected(
                        ResponseContentUnexpected {
                            status: local_var_status,
                            text: String::from_utf8_lossy(&body).to_string(),
                        },
                    ))),
                },
                Err(error) => Err(Error::Response(ResponseError::PayloadError {
                    status: local_var_status,
                    error,
                })),
            }
            
        }
    }
    
    
    
    async fn del_volume_snapshot(&self, volume_id: &uuid::Uuid, snapshot_id: &uuid::Uuid) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/volumes/{volume_id}/snapshots/{snapshot_id}", configuration.base_path, volume_id=volume_id.to_string(), snapshot_id=snapshot_id.to_string());
        let mut local_var_req_builder = hyper::Request::builder().method(hyper::Method::DELETE);

        
        
        
        
        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(hyper::header::USER_AGENT, local_var_user_agent.clone());
        }
        
        
        
        
        
        
        
        let body = hyper::Body::empty();
        
        let request = local_var_req_builder.uri(local_var_uri_str).header("content-type", "application/json").body(body).map_err(RequestError::BuildRequest)?;

        let local_var_resp = {
            let mut client_service = configuration.client_service.lock().await.clone();
            client_service
                .ready()
                .await
                .map_err(RequestError::NotReady)?
                .call(request)
                .await
                .map_err(RequestError::Request)?
        };
        let local_var_status = local_var_resp.status();
        
        if local_var_status.is_success() {
            
            
            
            let body = hyper::body::to_bytes(local_var_resp.into_body()).await.map_err(|e| ResponseError::PayloadError {
                status: local_var_status,
                error: e,
            })?;
            let local_var_content: () =
                serde_json::from_slice(&body).map_err(|e| {
                    ResponseError::Unexpected(ResponseContentUnexpected {
                        status: local_var_status,
                        text: e.to_string(),
                    })
                })?;
            Ok(ResponseContent { status: local_var_status, body: local_var_content })
            
            
            
        } else {
            match hyper::body::to_bytes(local_var_resp.into_body()).await {
                Ok(body) => match serde_json::from_slice::<crate::models::RestJsonError>(&body) {
                    Ok(error) => Err(Error::Response(ResponseError::Expected(ResponseContent {
                        status: local_var_status,
                        body: error,
                    }))),
                    Err(_) => Err(Error::Response(ResponseError::Unexpected(
                        ResponseContentUnexpected {
                            status: local_var_status,
                            text: String::from_utf8_lossy(&body).to_string(),
                        },
                    ))),
                },
                Err(error) => Err(Error::Response(ResponseError::PayloadError {
                    status: local_var_status,
                    error,
                })),
            }
            
        }
    }
    
    
    
    async fn get_volume_snapshots(&self, volume_id: &uuid::Uuid) -> Result<ResponseContent<Vec<crate::models::VolumeSnapshot>>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/volumes/{volume_id}/snapshots", configuration.base_path, volume_id=volume_id.to_string());
        let mut local_var_req_builder = hyper::Request::builder().method(hyper::Method::GET);

        
        
        
        
        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(hyper::header::USER_AGENT, local_var_user_agent.clone());
        }
        
        
        
        
        
        
        
        let body = hyper::Body::empty();
        
        let request = local_var_req_builder.uri(local_var_uri_str).header("content-type", "application/json").body(body).map_err(RequestError::BuildRequest)?;

        let local_var_resp = {
            let mut client_service = configuration.client_service.lock().await.clone();
            client_service
                .ready()
                .await
                .map_err(RequestError::NotReady)?
                .call(request)
                .await
                .map_err(RequestError::Request)?
        };
        let local_var_status = local_var_resp.status();
        
        if local_var_status.is_success() {
            
            
            
            let body = hyper::body::to_bytes(local_var_resp.into_body()).await.map_err(|e| ResponseError::PayloadError {
                status: local_var_status,
                error: e,
            })?;
            let local_var_content: Vec<crate::models::VolumeSnapshot> =
                serde_json::from_slice(&body).map_err(|e| {
                    ResponseError::Unexpected(ResponseContentUnexpected {
                        status: local_var_status,
                        text: e.to_string(),
                    })
                })?;
            Ok(ResponseContent { status: local_var_status, body: local_var_content })
            
            
            
        } else {
            match hyper::body::to_bytes(local_var_resp.into_body()).await {
                Ok(body) => match serde_json::from_slice::<crate::models::RestJsonError>(&body) {
                    Ok(error) => Err(Error::Response(ResponseError::Expected(ResponseContent {
                        status: local_var_status,
                        body: error,
                    }))),
                    Err(_) => Err(Error::Response(ResponseError::Unexpected(
                        ResponseContentUnexpected {
                            status: local_var_status,
                            text: String::from_utf8_lossy(&body).to_string(),
                        },
                    ))),
                },
                Err(error) => Err(Error::Response(ResponseError::PayloadError {
                    status: local_var_status,
                    error,
                })),
            }
            
        }
    }
    
    
    
    async fn del_snapshot(&self, snapshot_id: &uuid::Uuid) -> Result<ResponseContent<()>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/snapshots/{snapshot_id}", configuration.base_path, snapshot_id=snapshot_id.to_string());
        let mut local_var_req_builder = hyper::Request::builder().method(hyper::Method::DELETE);

        
        
        
        
        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(hyper::header::USER_AGENT, local_var_user_agent.clone());
        }
        
        
        
        
        
        
        
        let body = hyper::Body::empty();
        
        let request = local_var_req_builder.uri(local_var_uri_str).header("content-type", "application/json").body(body).map_err(RequestError::BuildRequest)?;

        let local_var_resp = {
            let mut client_service = configuration.client_service.lock().await.clone();
            client_service
                .ready()
                .await
                .map_err(RequestError::NotReady)?
                .call(request)
                .await
                .map_err(RequestError::Request)?
        };
        let local_var_status = local_var_resp.status();
        
        if local_var_status.is_success() {
            
            
            
            let body = hyper::body::to_bytes(local_var_resp.into_body()).await.map_err(|e| ResponseError::PayloadError {
                status: local_var_status,
                error: e,
            })?;
            let local_var_content: () =
                serde_json::from_slice(&body).map_err(|e| {
                    ResponseError::Unexpected(ResponseContentUnexpected {
                        status: local_var_status,
                        text: e.to_string(),
                    })
                })?;
            Ok(ResponseContent { status: local_var_status, body: local_var_content })
            
            
            
        } else {
            match hyper::body::to_bytes(local_var_resp.into_body()).await {
                Ok(body) => match serde_json::from_slice::<crate::models::RestJsonError>(&body) {
                    Ok(error) => Err(Error::Response(ResponseError::Expected(ResponseContent {
                        status: local_var_status,
                        body: error,
                    }))),
                    Err(_) => Err(Error::Response(ResponseError::Unexpected(
                        ResponseContentUnexpected {
                            status: local_var_status,
                            text: String::from_utf8_lossy(&body).to_string(),
                        },
                    ))),
                },
                Err(error) => Err(Error::Response(ResponseError::PayloadError {
                    status: local_var_status,
                    error,
                })),
            }
            
        }
    }
    
    
    
    async fn get_snapshots(&self, ) -> Result<ResponseContent<Vec<crate::models::VolumeSnapshot>>, Error<crate::models::RestJsonError>> {
    
        let configuration = &self.configuration;

        let local_var_uri_str = format!("{}/snapshots", configuration.base_path);
        let mut local_var_req_builder = hyper::Request::builder().method(hyper::Method::GET);

        
        
        
        
        if let Some(ref local_var_user_agent) = configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(hyper::header::USER_AGENT, local_var_user_agent.clone());
        }
        
        
        
        
        
        
        
        let body = hyper::Body::empty();
        
        let request = local_var_req_builder.uri(local_var_uri_str).header("content-type", "application/json").body(body).map_err(RequestError::BuildRequest)?;

        let local_var_resp = {
            let mut client_service = configuration.client_service.lock().await.clone();
            client_service
                .ready()
                .await
                .map_err(RequestError::NotReady)?
                .call(request)
                .await
                .map_err(RequestError::Request)?
        };
        let local_var_status = local_var_resp.status();
        
        if local_var_status.is_success() {
            
            
            
            let body = hyper::body::to_bytes(local_var_resp.into_body()).await.map_err(|e| ResponseError::PayloadError {
                status: local_var_status,
                error: e,
            })?;
            let local_var_content: Vec<crate::models::VolumeSnapshot> =
                serde_json::from_slice(&body).map_err(|e| {
                    ResponseError::Unexpected(ResponseContentUnexpected {
                        status: local_var_status,
                        text: e.to_string(),
                    })
                })?;
            Ok(ResponseContent { status: local_var_status, body: local_var_content })
            
            
            
        } else {
            match hyper::body::to_bytes(local_var_resp.into_body()).await {
                Ok(body) => match serde_json::from_slice::<crate::models::RestJsonError>(&body) {
                    Ok(error) => Err(Error::Response(ResponseError::Expected(ResponseContent {
                        status: local_var_status,
                        body: error,
                    }))),
                    Err(_) => Err(Error::Response(ResponseError::Unexpected(
                        ResponseContentUnexpected {
                            status: local_var_status,
                            text: String::from_utf8_lossy(&body).to_string(),
                        },
                    ))),
                },
                Err(error) => Err(Error::Response(ResponseError::PayloadError {
                    status: local_var_status,
                    error,
                })),
            }
            
        }
    }
    
    
}