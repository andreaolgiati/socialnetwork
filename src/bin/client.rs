use socialnetwork::server::social_network::social_network_service_client::SocialNetworkServiceClient;
use socialnetwork::server::social_network::*;
use std::time::Duration;
use tokio::time::sleep;
use rand::Rng;
use clap::Parser;

#[derive(Parser)]
#[command(name = "social-network-client")]
#[command(about = "A client simulator for the social network gRPC service")]
struct Args {
    /// User ID for this client
    #[arg(short, long)]
    user_id: u64,

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
        println!("Client {} connected to Social Network gRPC Server at {}", args.user_id, args.server);
    }
    
    // Other users to interact with
    let other_users = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut rng = rand::thread_rng();
    
    // Simulate random actions
    for action_num in 0..args.actions {
        let action = rng.gen_range(0..3); // 0=follow, 1=unfollow, 2=check
        
        match action {
            0 => {
                // Follow a random user
                let followee = other_users[rng.gen_range(0..other_users.len())];
                if followee != args.user_id {
                    let request = tonic::Request::new(FollowRequest {
                        follower_id: args.user_id,
                        followee_id: followee,
                    });
                    
                    match client.follow(request).await {
                        Ok(response) => {
                            let resp = response.into_inner();
                            if resp.success {
                                if args.verbose {
                                    println!("[{}] Followed user {} (new: {})", args.user_id, followee, resp.was_new_follow);
                                }
                            } else {
                                println!("[{}] Failed to follow {}: {}", args.user_id, followee, resp.error_message);
                            }
                        }
                        Err(e) => println!("[{}] Error following: {}", args.user_id, e),
                    }
                    
                    // Commit after follow
                    let _ = client.commit(tonic::Request::new(CommitRequest {})).await;
                }
            }
            1 => {
                // Unfollow a random user
                let followee = other_users[rng.gen_range(0..other_users.len())];
                if followee != args.user_id {
                    let request = tonic::Request::new(UnfollowRequest {
                        follower_id: args.user_id,
                        followee_id: followee,
                    });
                    
                    match client.unfollow(request).await {
                        Ok(response) => {
                            let resp = response.into_inner();
                            if resp.success {
                                if args.verbose {
                                    println!("[{}] Unfollowed user {} (was following: {})", args.user_id, followee, resp.was_unfollowed);
                                }
                            } else {
                                println!("[{}] Failed to unfollow {}: {}", args.user_id, followee, resp.error_message);
                            }
                        }
                        Err(e) => println!("[{}] Error unfollowing: {}", args.user_id, e),
                    }
                    
                    // Commit after unfollow
                    let _ = client.commit(tonic::Request::new(CommitRequest {})).await;
                }
            }
            2 => {
                // Check if following a random user
                let followee = other_users[rng.gen_range(0..other_users.len())];
                if followee != args.user_id {
                    let request = tonic::Request::new(IsFollowingRequest {
                        follower_id: args.user_id,
                        followee_id: followee,
                        version: None,
                    });
                    
                    match client.is_following(request).await {
                        Ok(response) => {
                            let is_following = response.into_inner().is_following;
                            if args.verbose {
                                println!("[{}] Is following user {}: {}", args.user_id, followee, is_following);
                            }
                        }
                        Err(e) => println!("[{}] Error checking follow status: {}", args.user_id, e),
                    }
                }
            }
            _ => unreachable!(),
        }
        
        // Random delay between actions
        let delay = rng.gen_range(args.min_delay..=args.max_delay);
        sleep(Duration::from_millis(delay)).await;
    }
    
    // Show final state for this user
    if args.verbose {
        println!("\n[{}] === Final State ===", args.user_id);
        
        // Get followers
        let followers_request = tonic::Request::new(GetFollowersRequest { user_id: args.user_id });
        if let Ok(response) = client.get_followers(followers_request).await {
            let followers = response.into_inner().follower_ids;
            println!("[{}] Has {} followers: {:?}", args.user_id, followers.len(), followers);
        }
        
        // Get followees
        let followees_request = tonic::Request::new(GetFolloweesRequest { user_id: args.user_id });
        if let Ok(response) = client.get_followees(followees_request).await {
            let followees = response.into_inner().followee_ids;
            println!("[{}] Follows {} users: {:?}", args.user_id, followees.len(), followees);
        }
    }
    
    println!("[{}] Client finished after {} actions", args.user_id, args.actions);
    Ok(())
} 