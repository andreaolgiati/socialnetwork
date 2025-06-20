# Social Network with Versioning

A Rust implementation of a social network graph with versioning capabilities, featuring a gRPC service and client simulators.

## Features

- **Versioned Social Network**: Track follow/unfollow relationships with full history
- **gRPC Service**: High-performance network service using Tonic
- **Client Simulators**: Multiple client simulators for testing and demonstration
- **Thread-Safe**: Concurrent access with proper synchronization

## Architecture

### Core Library (`src/lib.rs`)
- `SocialNetwork`: Main data structure with versioning
- `FollowInterval`: Represents a follow relationship with start/end versions
- All core functionality is exposed as a library

### gRPC Service (`src/server.rs`)
- Implements the `SocialNetworkService` trait
- Thread-safe with `Mutex<SocialNetwork>`
- Handles all network operations asynchronously

### Binaries
- `socialnetwork`: Demo of the core library functionality
- `server`: gRPC server (listens on `[::1]:50051`)
- `client`: Client simulator that generates random user interactions

## Quick Start

### 1. Build the Project
```bash
cargo build
```

### 2. Run the Demo
```bash
cargo run --bin socialnetwork
```

### 3. Start the gRPC Server
```bash
cargo run --bin server
```

### 4. Run Client Simulator
In another terminal:
```bash
cargo run --bin client
```

## API Usage

### Core Library
```rust
use socialnetwork::SocialNetwork;

let mut network = SocialNetwork::new();

// Follow a user
network.follow(1, 2)?;
let version = network.commit();

// Check if following
let is_following = network.is_following(1, 2, Some(version));

// Unfollow
network.unfollow(1, 2)?;
network.commit();
```

### gRPC Service
The service provides these RPC methods:
- `Follow(follower_id, followee_id)` → `(success, error_message, was_new_follow)`
- `Unfollow(follower_id, followee_id)` → `(success, error_message, was_unfollowed)`
- `IsFollowing(follower_id, followee_id, version?)` → `is_following`
- `GetFollowers(user_id)` → `follower_ids[]`
- `GetFollowees(user_id)` → `followee_ids[]`
- `GetFollowerCount(user_id)` → `count`
- `GetFolloweeCount(user_id)` → `count`
- `Commit()` → `version`
- `GetCurrentVersion()` → `version`

## Versioning

The social network maintains a complete history of all follow/unfollow actions:

- Each action increments the version counter
- Follow relationships are stored as intervals with start/end versions
- You can query the state at any historical version
- `u64::MAX` represents an "open" interval (currently following)

## Testing

Run the test suite:
```bash
cargo test
```

The tests cover:
- Basic follow/unfollow operations
- Versioning functionality
- Multiple user relationships
- Edge cases and error conditions

## Project Structure

```
socialnetwork/
├── src/
│   ├── lib.rs          # Core library
│   ├── server.rs       # gRPC server implementation
│   ├── main.rs         # Demo binary
│   └── bin/
│       ├── server.rs   # gRPC server binary
│       └── client.rs   # Client simulator
├── proto/
│   └── social_network.proto  # Protocol buffer definitions
├── build.rs            # Build script for proto compilation
└── Cargo.toml          # Dependencies and configuration
```

## Dependencies

- **tonic**: gRPC framework
- **prost**: Protocol buffer code generation
- **tokio**: Async runtime
- **rand**: Random number generation for client simulator
- **serde**: Serialization (for future JSON API)

## Performance Considerations

- The current implementation uses in-memory storage
- For production use, consider adding persistence (database)
- The `Mutex` provides thread safety but may become a bottleneck under high concurrency
- Consider using `RwLock` or more sophisticated concurrency patterns for better performance

## Future Enhancements

- Database persistence (PostgreSQL, Redis)
- GraphQL API in addition to gRPC
- Real-time notifications
- Analytics and metrics
- Load balancing and clustering
- Authentication and authorization
