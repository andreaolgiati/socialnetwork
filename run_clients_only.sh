#!/bin/bash

# Number of clients to run (default: 5)
NUM_CLIENTS=${1:-5}

echo "Starting $NUM_CLIENTS clients..."

# Start multiple clients in parallel
for i in $(seq 1 $NUM_CLIENTS); do
    echo "Starting client $i..."
    cargo run --bin client -- --user-id $i &
    CLIENT_PIDS[$i]=$!
done

echo "All clients started. Press Ctrl+C to stop all clients."

# Function to cleanup processes on exit
cleanup() {
    echo -e "\nStopping all clients..."
    
    # Kill all client processes
    for pid in "${CLIENT_PIDS[@]}"; do
        if kill -0 $pid 2>/dev/null; then
            kill $pid
        fi
    done
    
    echo "All clients stopped."
    exit 0
}

# Set up signal handlers
trap cleanup SIGINT SIGTERM

# Wait for all client processes to finish
for pid in "${CLIENT_PIDS[@]}"; do
    wait $pid
done

echo "All clients finished." 