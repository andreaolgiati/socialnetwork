use std::collections::{HashMap, HashSet};

//use rand::{distributions::uniform::SampleUniform, seq::index::sample};

pub mod server;

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
#[derive(Debug)]
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
    pub fn follow(&mut self, follower_id: u64, followee_id: u64) -> Result<bool, String> {
        if follower_id == followee_id {
            return Err("Users cannot follow themselves".to_string());
        }

        // Add to follows
        self.follows.entry(follower_id).or_insert_with(HashSet::new).insert(followee_id);

        // Add to is_followed
        self.is_followed.entry(followee_id).or_insert_with(HashSet::new).insert(follower_id);
        
       
        // If already following (i.e., last interval is open), do nothing
        if let Some(intervals) = self.follow_intervals.get_mut(&(follower_id, followee_id)) {
            let last = intervals.last_mut().expect("Follow intervals should not be empty");
            if last.follow_end == u64::MAX {
                return Ok(false);
            } else if last.follow_end == self.version {
                // this is the case where a unfollows b, then they follow again in the same version
                last.follow_end = u64::MAX;
                return Ok(false);
            } else if last.follow_end < self.version {
                // do nothing
            }
        }

         // Create follow interval
        let interval = FollowInterval::new(self.version);
        self.follow_intervals
            .entry((follower_id, followee_id))
            .or_insert_with(Vec::new)
            .push(interval);

        Ok(true)
    }

    /// Unfollow a user
    pub fn unfollow(&mut self, follower_id: u64, followee_id: u64) -> Result<bool, String> {
        if follower_id == followee_id {
            return Err("Users cannot unfollow themselves".to_string());
        }
        if !self.follows.contains_key(&follower_id) {
            return Ok(false);
        }
        if !self.follows[&follower_id].contains(&followee_id) {
            return Ok(false);
        }

        // Remove from follows
        self.follows.get_mut(&follower_id).unwrap().remove(&followee_id);

        // Remove from is_followed
        self.is_followed.get_mut(&followee_id).unwrap().remove(&follower_id);

        // Find the follow intervals for the follower and followee
        let follow_intervals = self.follow_intervals.get_mut(&(follower_id, followee_id));
        
        match follow_intervals {
            Some(follow_intervals) => {
                
                // Assert that the follow intervals are not empty
                assert!(!follow_intervals.is_empty());

                let last_interval = follow_intervals.last_mut();
                match last_interval {
                    Some(interval) => {
                        if interval.follow_end == u64::MAX {
                            interval.follow_end = self.version;
                            return Ok(true);
                        } else if interval.follow_end == self.version {
                            interval.follow_end = u64::MAX;
                            return Ok(true);
                        } else {
                            return Err("Invalid follow interval".to_string());
                        }
                    }
                    None => {
                        return Err("Invalid follow interval".to_string());
                    }
                }
            }
            None => {
                return Ok(false);
            }
        }
    }

    /// Check if follower is following followee (use current version if not specified)
    pub fn is_following(&self, follower_id: u64, followee_id: u64, version: Option<u64>) -> bool {
        let version = version.unwrap_or(self.version);
        if version > self.version {
            return false;
        }
        
        match self.follow_intervals.get(&(follower_id, followee_id)) {
            // go back to checking any of the intervals
            Some(follow_intervals) => {
                return follow_intervals.iter().any(|interval| interval.is_active(version));
            }
            None => {
                return false;
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_network() {
        let network = SocialNetwork::new();
        assert_eq!(network.current_version(), 0);
    }

    #[test]
    fn test_follow() {
        let mut network = SocialNetwork::new();
        
        // Test successful follow
        assert!(network.follow(1, 2).is_ok());
        assert!(network.is_following(1, 2, None));
        
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
        assert!(network.unfollow(1, 2).unwrap());
        // Commit to advance version so the unfollow takes effect
        network.commit();
        assert!(!network.is_following(1, 2, None));
        
        // Test unfollowing when not following
        assert!(!network.unfollow(1, 2).unwrap());
        
        // Test self-unfollow prevention
        assert!(network.unfollow(1, 1).is_err());
    }

    #[test]
    fn test_versioning() {
        let mut network = SocialNetwork::new();
        
        // Follow at version 0
        network.follow(1, 2).unwrap();
        assert_eq!(network.current_version(), 0);
        assert!(network.is_following(1, 2, Some(0)));
        
        // Commit to version 1
        let version = network.commit();
        assert_eq!(version, 1);
        assert_eq!(network.current_version(), 1);
        assert!(network.is_following(1, 2, Some(1)));
        
        // Unfollow at version 1 (interval ends at version 1)
        network.unfollow(1, 2).unwrap();
        // At version 1, they are still following (interval [0,1] is active at version 1)
        assert!(network.is_following(1, 2, Some(1)));
        
        // Commit to version 2
        network.commit();
        // At version 2, they are no longer following (interval [0,1] is not active at version 2)
        assert!(!network.is_following(1, 2, Some(2)));
        
        // Check historical versions
        assert!(network.is_following(1, 2, Some(0))); // Was following at version 0
        assert!(network.is_following(1, 2, Some(1))); // Was still following at version 1
        assert!(!network.is_following(1, 2, Some(2))); // Stopped following at version 2
    }

    #[test]
    fn test_multiple_relationships() {
        let mut network = SocialNetwork::new();
        
        // User 1 follows users 2, 3, 4
        network.follow(1, 2).unwrap();
        network.follow(1, 3).unwrap();
        network.follow(1, 4).unwrap();
        
        // User 2 follows user 1
        network.follow(2, 1).unwrap();
        
        let followees = network.get_followees(1);
        assert_eq!(followees.len(), 3);
        assert!(followees.contains(&2));
        assert!(followees.contains(&3));
        assert!(followees.contains(&4));
        
        let followers = network.get_followers(1);
        assert_eq!(followers.len(), 1);
        assert!(followers.contains(&2));
    }

    #[test]
    fn test_get_followers_and_followees() {
        let mut network = SocialNetwork::new();
        
        // Setup multiple relationships
        network.follow(1, 2).unwrap();
        network.follow(1, 3).unwrap();
        network.follow(2, 1).unwrap();
        network.follow(4, 1).unwrap();
        
        let followers = network.get_followers(1);
        assert_eq!(followers.len(), 2);
        assert!(followers.contains(&2));
        assert!(followers.contains(&4));
        
        let followees = network.get_followees(1);
        assert_eq!(followees.len(), 2);
        assert!(followees.contains(&2));
        assert!(followees.contains(&3));
        
        // Test non-existent user
        assert_eq!(network.get_followers(999).len(), 0);
        assert_eq!(network.get_followees(999).len(), 0);
    }

    #[test]
    fn test_nonexistent_version() {
        let mut network = SocialNetwork::new();
        network.follow(1, 2).unwrap();
        
        // Should return false for versions beyond current
        assert!(!network.is_following(1, 2, Some(999)));
    }

    #[test]
    fn test_refollow() {
        let mut network = SocialNetwork::new();
        
        // Follow, unfollow, then follow again
        network.follow(1, 2).unwrap();
        network.commit();
        network.unfollow(1, 2).unwrap();
        network.commit();
        
        // Refollow should work
        assert!(network.follow(1, 2).unwrap());
        assert!(network.is_following(1, 2, None));
    }
} 