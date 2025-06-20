#!/bin/bash

# Number of clients to run (default: 5)
NUM_CLIENTS=${1:-5}

echo "Starting Social Network gRPC Server..."
# Start the server in the background
cargo run --bin server &
SERVER_PID=$!

# Wait a moment for server to start
sleep 2

echo "Starting $NUM_CLIENTS clients..."

# Start multiple clients in parallel
for i in $(seq 1 $NUM_CLIENTS); do
    echo "Starting client $i..."
    cargo run --bin client -- --user-id $i &
    CLIENT_PIDS[$i]=$!
done

echo "All clients started. Press Ctrl+C to stop all processes."

# Function to cleanup processes on exit
cleanup() {
    echo -e "\nStopping all processes..."
    
    # Kill all client processes
    for pid in "${CLIENT_PIDS[@]}"; do
        if kill -0 $pid 2>/dev/null; then
            kill $pid
        fi
    done
    
    # Kill server process
    if kill -0 $SERVER_PID 2>/dev/null; then
        kill $SERVER_PID
    fi
    
    echo "All processes stopped."
    exit 0
}

# Set up signal handlers
trap cleanup SIGINT SIGTERM

# Wait for all client processes to finish
for pid in "${CLIENT_PIDS[@]}"; do
    wait $pid
done

# Stop server
cleanup 