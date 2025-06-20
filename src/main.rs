use socialnetwork::SocialNetwork;

fn main() {
    let mut network = SocialNetwork::new();
    
    println!("=== Social Network Graph Demo ===");
    
    // Initial commit
    let v0 = network.commit();
    println!("Initial version: {}", v0);
    
    // User 1 follows user 2
    network.follow(1, 2).unwrap();
    let v1 = network.commit();
    println!("User 1 follows user 2 -> Version {}", v1);
    
    // User 1 follows user 3
    network.follow(1, 3).unwrap();
    let v2 = network.commit();
    println!("User 1 follows user 3 -> Version {}", v2);
    
    // User 2 follows user 1
    network.follow(2, 1).unwrap();
    let v3 = network.commit();
    println!("User 2 follows user 1 -> Version {}", v3);
    
    // User 1 unfollows user 3
    network.unfollow(1, 3).unwrap();
    let v4 = network.commit();
    println!("User 1 unfollows user 3 -> Version {}", v4);
    
    // Check relationships at different versions
    println!("\n=== Relationship History ===");
    println!("User 1 following User 2:");
    for version in v0..=v4 {
        let following = network.is_following(1, 2, Some(version));
        println!("  Version {}: {}", version, following);
    }
    
    println!("\nUser 1 following User 3:");
    for version in v0..=v4 {
        let following = network.is_following(1, 3, Some(version));
        println!("  Version {}: {}", version, following);
    }
    
    println!("\nUser 2 following User 1:");
    for version in v0..=v4 {
        let following = network.is_following(2, 1, Some(version));
        println!("  Version {}: {}", version, following);
    }
    
    // Current state
    println!("\n=== Current State ===");
    println!("User 1 followers: {:?}", network.get_followers(1));
    println!("User 1 followees: {:?}", network.get_followees(1));
    println!("User 2 followers: {:?}", network.get_followers(2));
    println!("User 2 followees: {:?}", network.get_followees(2));
} 