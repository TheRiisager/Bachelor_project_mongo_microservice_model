use std::sync::Arc;

use tonic::{Request, Response, Status};
use crate::mongo::{MongoDBService, DBEntry};
use crate::virk_data::virk_data_server::VirkData;
use crate::virk_data::{EntryUpdateRequest, EntryUpdateResponse, GetAllIdsRequest, GetAllIdsResponse};

pub struct GRPCService{
    mongo: Arc<MongoDBService>
}

impl GRPCService {
    pub fn new(mongo: Arc<MongoDBService>) -> Self {
        GRPCService { mongo }
    }
}

#[tonic::async_trait]
impl VirkData for GRPCService {
    async fn add_or_update_entry(&self, request: Request<EntryUpdateRequest>) -> Result<Response<EntryUpdateResponse>, Status> {
        let req = request.into_inner();
        let entry = DBEntry {
            id: req.id,
            col1: req.col1,
            col2: req.col2
        };
        let res = *&self.mongo
            .save_entry(entry)
            .await
            .expect("mongo error");
        Ok(Response::new(
            EntryUpdateResponse {
                id: res
            }
        ))
    }

    async fn get_all_ids(&self, _request: Request<GetAllIdsRequest>) -> Result<Response<GetAllIdsResponse>, Status> {
        let res = &self.mongo.get_all_ids()
            .await
            .expect("mongo error");
        Ok(Response::new(GetAllIdsResponse { ids: res.to_vec() }))
    }
}