use std::sync::Mutex;
use tonic::{Request, Response, Status};
use crate::SocialNetwork;

// Include the generated proto code
pub mod social_network {
    tonic::include_proto!("social_network");
}

use social_network::social_network_service_server::{SocialNetworkService, SocialNetworkServiceServer};
use social_network::*;

pub struct SocialNetworkServer {
    network: Mutex<SocialNetwork>,
}

impl SocialNetworkServer {
    pub fn new() -> Self {
        Self {
            network: Mutex::new(SocialNetwork::new()),
        }
    }
}

#[tonic::async_trait]
impl SocialNetworkService for SocialNetworkServer {
    async fn follow(
        &self,
        request: Request<FollowRequest>,
    ) -> Result<Response<FollowResponse>, Status> {
        let req = request.into_inner();
        let mut network = self.network.lock().unwrap();
        
        match network.follow(req.follower_id, req.followee_id) {
            Ok(was_new_follow) => {
                Ok(Response::new(FollowResponse {
                    success: true,
                    error_message: String::new(),
                    was_new_follow,
                }))
            }
            Err(error_msg) => {
                Ok(Response::new(FollowResponse {
                    success: false,
                    error_message: error_msg,
                    was_new_follow: false,
                }))
            }
        }
    }

    async fn unfollow(
        &self,
        request: Request<UnfollowRequest>,
    ) -> Result<Response<UnfollowResponse>, Status> {
        let req = request.into_inner();
        let mut network = self.network.lock().unwrap();
        
        match network.unfollow(req.follower_id, req.followee_id) {
            Ok(was_unfollowed) => {
                Ok(Response::new(UnfollowResponse {
                    success: true,
                    error_message: String::new(),
                    was_unfollowed,
                }))
            }
            Err(error_msg) => {
                Ok(Response::new(UnfollowResponse {
                    success: false,
                    error_message: error_msg,
                    was_unfollowed: false,
                }))
            }
        }
    }

    async fn is_following(
        &self,
        request: Request<IsFollowingRequest>,
    ) -> Result<Response<IsFollowingResponse>, Status> {
        let req = request.into_inner();
        let network = self.network.lock().unwrap();
        
        let is_following = network.is_following(req.follower_id, req.followee_id, req.version);
        
        Ok(Response::new(IsFollowingResponse { is_following }))
    }

    async fn get_followers(
        &self,
        request: Request<GetFollowersRequest>,
    ) -> Result<Response<GetFollowersResponse>, Status> {
        let req = request.into_inner();
        let network = self.network.lock().unwrap();
        
        let followers = network.get_followers(req.user_id);
        
        Ok(Response::new(GetFollowersResponse {
            follower_ids: followers,
        }))
    }

    async fn get_followees(
        &self,
        request: Request<GetFolloweesRequest>,
    ) -> Result<Response<GetFolloweesResponse>, Status> {
        let req = request.into_inner();
        let network = self.network.lock().unwrap();
        
        let followees = network.get_followees(req.user_id);
        
        Ok(Response::new(GetFolloweesResponse {
            followee_ids: followees,
        }))
    }

    async fn commit(
        &self,
        _request: Request<CommitRequest>,
    ) -> Result<Response<CommitResponse>, Status> {
        let mut network = self.network.lock().unwrap();
        let version = network.commit();
        
        Ok(Response::new(CommitResponse { version }))
    }

    async fn get_current_version(
        &self,
        _request: Request<GetCurrentVersionRequest>,
    ) -> Result<Response<GetCurrentVersionResponse>, Status> {
        let network = self.network.lock().unwrap();
        let version = network.current_version();
        
        Ok(Response::new(GetCurrentVersionResponse { version }))
    }
}

pub fn create_server() -> SocialNetworkServiceServer<SocialNetworkServer> {
    SocialNetworkServiceServer::new(SocialNetworkServer::new())
} 