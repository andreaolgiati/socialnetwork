use socialnetwork::server::social_network::social_network_service_client::SocialNetworkServiceClient;
use socialnetwork::server::social_network::*;
use std::time::Duration;
use tokio::time::sleep;
use rand::Rng;
use clap::Parser;

#[derive(Debug)]
enum Action {
    Follow,
    Unfollow,
    Check,
}

impl Action {
    fn random() -> Action {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..3) {
            0 => Action::Follow,
            1 => Action::Unfollow,
            2 => Action::Check,
            _ => unreachable!(),
        }
    }
}

#[derive(Parser)]
#[command(name = "social-network-client")]
#[command(about = "A client simulator for the social network gRPC service")]
struct Args {
    /// User ID for this client
    #[arg(short, long)]
    max_user_id: u64,

    /// Number of actions to perform
    #[arg(short, long, default_value_t = 1000)]
    actions: u32,

    /// Minimum delay between actions (milliseconds)
    #[arg(long, default_value_t = 1)]
    min_delay: u64,

    /// Maximum delay between actions (milliseconds)
    #[arg(long, default_value_t = 10)]
    max_delay: u64,

    /// Server address
    #[arg(long, default_value = "http://[::1]:50051")]
    server: String,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    let mut client = SocialNetworkServiceClient::connect(args.server.clone()).await?;
    
    if args.verbose {
        println!("Client connected to Social Network gRPC Server at {}", args.server);
    }
    
    // Other users to interact with
    let mut rng = rand::thread_rng();
    
    // Simulate random actions
    for action_num in 0..args.actions {
        let action = Action::random();
        let follower_id = rng.gen_range(0..args.max_user_id);
        let followee_id = rng.gen_range(0..args.max_user_id);

        if follower_id == followee_id {
            continue;
        }
        
        match action {
            Action::Follow => {
                // Follow a random user
                let request = tonic::Request::new(FollowRequest {
                    follower_id: follower_id,
                    followee_id: followee_id,
                });
                    
                match client.follow(request).await {
                    Ok(response) => {
                        let resp = response.into_inner();
                        if resp.success {
                            if args.verbose {
                                println!("[{}] Followed user {} (new: {})", follower_id, followee_id, resp.was_new_follow);
                            }
                        } else {
                            println!("[{}] Failed to follow {}: {}", follower_id, followee_id, resp.error_message);
                        }
                    }
                    Err(e) => println!("[{}] Error following: {}", follower_id, e),
                }
                    
                // Commit after follow
                let _ = client.commit(tonic::Request::new(CommitRequest {})).await;
            }
            Action::Unfollow => {
                // Unfollow a random user
                let request = tonic::Request::new(UnfollowRequest {
                    follower_id: follower_id,
                    followee_id: followee_id,
                });
                    
                match client.unfollow(request).await {
                    Ok(response) => {
                        let resp = response.into_inner();
                        if resp.success {
                            if args.verbose {
                                println!("[{}] Unfollowed user {} (was following: {})", follower_id, followee_id, resp.was_unfollowed);
                            }
                        } else {
                            println!("[{}] Failed to unfollow {}: {}", follower_id, followee_id, resp.error_message);
                        }
                    }
                    Err(e) => println!("[{}] Error unfollowing: {}", follower_id, e),
                }
                    
                // Commit after unfollow
                let _ = client.commit(tonic::Request::new(CommitRequest {})).await;
            }
            Action::Check => {
                // Check if following a random user
                let request = tonic::Request::new(IsFollowingRequest {
                    follower_id: follower_id,
                    followee_id: followee_id,
                    version: None,
                });
                    
                match client.is_following(request).await {
                    Ok(response) => {
                        let is_following = response.into_inner().is_following;
                        if args.verbose {
                            println!("[{}] Is following user {}: {}", follower_id, followee_id, is_following);
                        }
                    }
                    Err(e) => println!("[{}] Error checking follow status: {}", follower_id, e),
                }
            }
        }
        
        // Random delay between actions
        let delay = rng.gen_range(args.min_delay..=args.max_delay);
        sleep(Duration::from_millis(delay)).await;
    }
    
    println!("Client finished after {} actions", args.actions);
    Ok(())
} 