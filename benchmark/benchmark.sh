#!/bin/sh

# The command you want to time
command="./mainrust ./hago.apk"

# Number of times to run the command
num_runs=100

# Variable to store the total time
total_time=0

# Run the command multiple times and calculate the total time
for ((i=1; i<=$num_runs; i++))
do
  
  # Use the 'time' command and direct its output to a variable
  { time_output=$( { time $command; } 2>&1 ); } 2>/dev/null

  # Extract the real time from the time output
  real_time=$(echo "$time_output" | grep real | awk '{print $2}')
  echo "Run $i - $real_time"
  # Add the real time to the total time
  total_time=$(echo "$total_time + $(echo "$real_time" | awk -F 'm|s' '{print ($1 * 60) + $2}')" | bc)
  done

# Calculate the average time
average_time=$(echo "scale=5; $total_time / $num_runs" | bc)

# Print the average time
echo "Average Time: $average_time seconds"

