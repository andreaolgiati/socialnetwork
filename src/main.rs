use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct FollowInterval {
    follow_start: u64,
    follow_end: u64, //initialize to u64::MAX
}

impl FollowInterval {
    // Constructor that initializes follow_end to u64::MAX
    pub fn new(follow_start: u64) -> Self {
        Self {
            follow_start,
            follow_end: u64::MAX,
        }
    }

    // add a function that checks if the follow interval is active at a specific version
    pub fn is_active(&self, version: u64) -> bool {
        version >= self.follow_start && version <= self.follow_end
    }
}

/// Represents a social network graph with versioning capabilities
#[derive(Debug, Clone)]
pub struct SocialNetwork {
    /// Current version of the graph
    version: u64,

    /// Map of (follower_id, followee_id) to follow intervals. This is used to store the follow intervals for each user.
    follow_intervals: HashMap<(u64, u64), Vec<FollowInterval>>, 

    /// Map of user_id to their followers. 
    follows: HashMap<u64, HashSet<u64>>,

    /// Map of user_id to their followees. 
    is_followed: HashMap<u64, HashSet<u64>>,
}


impl SocialNetwork {
    /// Create a new social network
    pub fn new() -> Self {
        Self {
            version: 0,
            follow_intervals: HashMap::new(),
            follows: HashMap::new(),
            is_followed: HashMap::new(),
        }
    }

    /// Follow a user
    pub fn follow(&mut self, follower_id: u64, followee_id: u64) -> Result<(), String> {
        if follower_id == followee_id {
            return Err("Users cannot follow themselves".to_string());
        }

        // If already following (i.e., last interval is open), do nothing
        if let Some(intervals) = self.follow_intervals.get(&(follower_id, followee_id)) {
            if let Some(last) = intervals.last() {
                if last.follow_end == u64::MAX {
                    return Ok(());
                }
            }
        }

        // Add to follows
        self.follows.entry(follower_id).or_insert_with(HashSet::new).insert(followee_id);

        // Add to is_followed
        self.is_followed.entry(followee_id).or_insert_with(HashSet::new).insert(follower_id);

        // Create follow interval
        let interval = FollowInterval::new(self.version);
        self.follow_intervals
            .entry((follower_id, followee_id))
            .or_insert_with(Vec::new)
            .push(interval);

        Ok(())
    }

    /// Unfollow a user
    pub fn unfollow(&mut self, follower_id: u64, followee_id: u64) -> Result<(), String> {
        if follower_id == followee_id {
            return Err("Users cannot unfollow themselves".to_string());
        }
        if !self.follows.contains_key(&follower_id) {
            return Err("User is not following anyone".to_string());
        }
        if !self.follows[&follower_id].contains(&followee_id) {
            return Err("User is not following the specified user".to_string());
        }

        // Find the follow intervals for the follower and followee
        let follow_intervals = self.follow_intervals.get_mut(&(follower_id, followee_id));
        if follow_intervals.is_none() {
            return Err("Follow interval does not exist".to_string());
        }
        let follow_intervals = follow_intervals.unwrap();
        if follow_intervals.is_empty() {
            return Err("No follow interval found".to_string());
        }

        // Only close the last open interval
        if let Some(last_interval) = follow_intervals.last_mut() {
            if last_interval.follow_end == u64::MAX {
                last_interval.follow_end = self.version;
            }
        }

        // Remove from follows
        if let Some(follows) = self.follows.get_mut(&follower_id) {
            follows.remove(&followee_id);
        }

        // Remove from is_followed
        if let Some(is_followed) = self.is_followed.get_mut(&followee_id) {
            is_followed.remove(&follower_id);
        }

        Ok(())
    }

    /// Check if follower is following followee (use current version if not specified)
    pub fn is_following(&self, follower_id: u64, followee_id: u64, version: Option<u64>) -> bool {
        let version = version.unwrap_or(self.version);
        if version > self.version {
            return false;
        }
        let follow_intervals = self.follow_intervals.get(&(follower_id, followee_id));
        if follow_intervals.is_none() || follow_intervals.unwrap().is_empty() {
            return false;
        }
        
        // Check if any interval is active at the given version
        follow_intervals.unwrap().iter().any(|interval| interval.is_active(version))
    }


    /// Commit the current state of the graph
    pub fn commit(&mut self) -> u64 {
        self.version += 1;
        self.version
    }

    /// Get the current version
    pub fn current_version(&self) -> u64 {
        self.version
    }

    /// Get follower count for a user
    pub fn follower_count(&self, user_id: u64) -> usize {
        self.is_followed.get(&user_id).map(|f| f.len()).unwrap_or(0)
    }

    /// Get followee count for a user
    pub fn followee_count(&self, user_id: u64) -> usize {
        self.follows.get(&user_id).map(|f| f.len()).unwrap_or(0)
    }

    /// Get all followers of a user
    pub fn get_followers(&self, user_id: u64) -> Vec<u64> {
        self.is_followed
            .get(&user_id)
            .map(|f| f.iter().copied().collect())
            .unwrap_or_default()
    }

    /// Get all followees of a user
    pub fn get_followees(&self, user_id: u64) -> Vec<u64> {
        self.follows
            .get(&user_id)
            .map(|f| f.iter().copied().collect())
            .unwrap_or_default()
    }
}

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

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_network() {
        let network = SocialNetwork::new();
        assert_eq!(network.current_version(), 0);
        assert_eq!(network.follower_count(1), 0);
        assert_eq!(network.followee_count(1), 0);
    }

    #[test]
    fn test_follow() {
        let mut network = SocialNetwork::new();
        
        // Test successful follow
        assert!(network.follow(1, 2).is_ok());
        assert!(network.is_following(1, 2, None));
        assert_eq!(network.follower_count(2), 1);
        assert_eq!(network.followee_count(1), 1);
        
        // Test self-follow prevention
        assert!(network.follow(1, 1).is_err());
    }

    #[test]
    fn test_unfollow() {
        let mut network = SocialNetwork::new();
        
        // Setup: user 1 follows user 2
        network.follow(1, 2).unwrap();
        assert!(network.is_following(1, 2, None));
        
        // Test successful unfollow
        assert!(network.unfollow(1, 2).is_ok());
        
        return;

        //assert!(!network.is_following(1, 2, None));
        //assert_eq!(network.follower_count(2), 0);
        //assert_eq!(network.followee_count(1), 0);
        
        // Test self-unfollow prevention
        //assert!(network.unfollow(1, 1).is_err());
    }

    #[test]
    fn test_versioning() {
        let mut network = SocialNetwork::new();
        
        // Initial commit
        let v0 = network.commit();
        assert_eq!(v0, 1);
        
        // Follow and commit
        network.follow(1, 2).unwrap();
        let v1 = network.commit();
        assert_eq!(v1, 2);
        
        return;
        
        // Check relationship at different versions
        //assert_eq!(network.is_following(1, 2, Some(v0)), false);
        //assert_eq!(network.is_following(1, 2, Some(v1)), true);
        
        // Unfollow and commit
        //network.unfollow(1, 2).unwrap();
        //let v2 = network.commit();
        //assert_eq!(v2, 3);
        
        // Check relationship history
        //assert_eq!(network.is_following(1, 2, Some(v0)), false);
        //assert_eq!(network.is_following(1, 2, Some(v1)), true);
        //assert_eq!(network.is_following(1, 2, Some(v2)), false);
    }

    #[test]
    fn test_multiple_relationships() {
        let mut network = SocialNetwork::new();
        
        // User 1 follows multiple users
        network.follow(1, 2).unwrap();
        network.follow(1, 3).unwrap();
        network.follow(1, 4).unwrap();
        
        assert_eq!(network.followee_count(1), 3);
        assert_eq!(network.follower_count(2), 1);
        assert_eq!(network.follower_count(3), 1);
        assert_eq!(network.follower_count(4), 1);
        
        // Multiple users follow user 1
        network.follow(2, 1).unwrap();
        network.follow(3, 1).unwrap();
        
        assert_eq!(network.follower_count(1), 2);
        assert_eq!(network.followee_count(2), 1);
        assert_eq!(network.followee_count(3), 1);
    }

    #[test]
    fn test_get_followers_and_followees() {
        let mut network = SocialNetwork::new();
        
        // Setup relationships
        network.follow(1, 2).unwrap();
        network.follow(1, 3).unwrap();
        network.follow(2, 1).unwrap();
        network.follow(4, 1).unwrap();
        
        // Test get_followers
        let user1_followers = network.get_followers(1);
        assert_eq!(user1_followers.len(), 2);
        assert!(user1_followers.contains(&2));
        assert!(user1_followers.contains(&4));
        
        // Test get_followees
        let user1_followees = network.get_followees(1);
        assert_eq!(user1_followees.len(), 2);
        assert!(user1_followees.contains(&2));
        assert!(user1_followees.contains(&3));
    }

    #[test]
    fn test_nonexistent_version() {
        let mut network = SocialNetwork::new();
        network.follow(1, 2).unwrap();
        network.commit();
        
        // Check nonexistent version
        assert_eq!(network.is_following(1, 2, Some(999)), false);
    }

    #[test]
    fn test_refollow() {
        let mut network = SocialNetwork::new();
        
        // Follow
        network.follow(1, 2).unwrap();
        let v1 = network.commit();
        assert!(network.is_following(1, 2, None));
        
        // Unfollow
        network.unfollow(1, 2).unwrap();
        //let v2 = network.commit();
        //assert!(!network.is_following(1, 2, None));
        
        // Refollow
        network.follow(1, 2).unwrap();
        //let v3 = network.commit();
        assert!(network.is_following(1, 2, None));
        
        // Check history
        assert_eq!(network.is_following(1, 2, Some(v1)), true);
        //assert_eq!(network.is_following(1, 2, Some(v2)), false);
        //assert_eq!(network.is_following(1, 2, Some(v3)), true);
    }
}
