#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]

use crate::{
    actix::server::{deserialize_option_stringified_list, deserialize_stringified_list},
    apis::{
        actix_server::{Body, NoContent, RestError},
        snapshots_api::actix::server,
    },
};
use actix_web::{
    web::{Json, Path, Query, ServiceConfig},
    FromRequest, HttpRequest,
};


/// Configure handlers for the Snapshots resource
pub fn configure<T: server::Snapshots + 'static>(cfg: &mut ServiceConfig) {
    cfg


       .service(
            actix_web::web::resource("/volumes/{volume_id}/snapshots/{snapshot_id}")
                .name("get_volume_snapshot")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_volume_snapshot::<T, A>))
       )

       .service(
            actix_web::web::resource("/volumes/{volume_id}/snapshots/{snapshot_id}")
                .name("put_volume_snapshot")
                .guard(actix_web::guard::Put())
                .route(actix_web::web::put().to(put_volume_snapshot::<T, A>))
       )

       .service(
            actix_web::web::resource("/volumes/{volume_id}/snapshots/{snapshot_id}")
                .name("del_volume_snapshot")
                .guard(actix_web::guard::Delete())
                .route(actix_web::web::delete().to(del_volume_snapshot::<T, A>))
       )

       .service(
            actix_web::web::resource("/volumes/{volume_id}/snapshots")
                .name("get_volume_snapshots")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_volume_snapshots::<T, A>))
       )

       .service(
            actix_web::web::resource("/snapshots/{snapshot_id}")
                .name("del_snapshot")
                .guard(actix_web::guard::Delete())
                .route(actix_web::web::delete().to(del_snapshot::<T, A>))
       )

       .service(
            actix_web::web::resource("/snapshots")
                .name("get_snapshots")
                .guard(actix_web::guard::Get())
                .route(actix_web::web::get().to(get_snapshots::<T, A>))
       );


}





















async fn get_volume_snapshot<T: server::Snapshots + 'static, A: FromRequest + 'static>(_token: A, path: Path<uuid::Uuid, uuid::Uuid>) -> Result<Json<crate::models::VolumeSnapshot>, RestError<crate::models::RestJsonError>> {
    T::get_volume_snapshot(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn put_volume_snapshot<T: server::Snapshots + 'static, A: FromRequest + 'static>(_token: A, path: Path<uuid::Uuid, uuid::Uuid>) -> Result<Json<crate::models::VolumeSnapshot>, RestError<crate::models::RestJsonError>> {
    T::put_volume_snapshot(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn del_volume_snapshot<T: server::Snapshots + 'static, A: FromRequest + 'static>(_token: A, path: Path<uuid::Uuid, uuid::Uuid>) -> Result<Json<()>, RestError<crate::models::RestJsonError>> {
    T::del_volume_snapshot(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn get_volume_snapshots<T: server::Snapshots + 'static, A: FromRequest + 'static>(_token: A, path: Path<uuid::Uuid>) -> Result<Json<Vec<crate::models::VolumeSnapshot>>, RestError<crate::models::RestJsonError>> {
    T::get_volume_snapshots(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn del_snapshot<T: server::Snapshots + 'static, A: FromRequest + 'static>(_token: A, path: Path<uuid::Uuid>) -> Result<Json<()>, RestError<crate::models::RestJsonError>> {
    T::del_snapshot(crate::apis::actix_server::Path(path.into_inner())).await.map(Json)
}




async fn get_snapshots<T: server::Snapshots + 'static, A: FromRequest + 'static>(_token: A) -> Result<Json<Vec<crate::models::VolumeSnapshot>>, RestError<crate::models::RestJsonError>> {
    T::get_snapshots().await.map(Json)
}


