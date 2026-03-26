#!/bin/bash

# Collect per-machine metrics and aggregate them into a single CSV.
#
# Usage:
#   Run from repository root so ./data/machines.txt can be resolved.
#
# Notes:
# - This currently simulates remote collection by generating random values.
# - Replace the simulated write with scp/ssh collection when ready.

# Quick workspace listing for debugging when run in CI.


# Array of machine names
mapfile -t MACHINES < ./data/machines.txt


# Local source file

SRC_FILE="metrics.csv"
TOTAL_FILE="all_metrics.csv"
DATE=$(date +%Y%m%d)
DATE_TIME=$(date +%Y%m%d_%H%M%S)

echo "Job Name: $JOB_NAME"
echo "Build ID: $BUILD_ID"
echo "Build URL: $BUILD_URL"
echo "Timestamp: $DATE_TIME"
ls ${TOTAL_FILE} 2>/dev/null || echo "No existing ${TOTAL_FILE} found, will create new one."
ls -l ./*${SRC_FILE}  # List existing metrics files for reference
# Loop through each machine and append its daily metric into TOTAL_FILE.
for machine in "${MACHINES[@]}"; do
    RANDOM_NUM=$((1 + RANDOM % 1000))  # Generate random number for demo
    echo "Fetching $SRC_FILE from $machine..."
    #touch "./${machine}_${DATE_TIME}_${SRC_FILE}"  # Create empty file to simulate scp
    echo  "${machine},${DATE},${RANDOM_NUM}" >"./${machine}_${DATE_TIME}_${SRC_FILE}"  # Simulated metric row; replace with actual scp command.
    status=$?
    if [ $status -eq 0 ]; then
        echo "✓ Success from $machine"
    else
        echo "✗ Failed from $machine"
    fi
    cat "./${machine}_${DATE_TIME}_${SRC_FILE}" >> "./${TOTAL_FILE}"  # Append to combined file
done
ls -l ./*${SRC_FILE}
echo "All metrics collected into ${TOTAL_FILE}"
echo "Contents of ${TOTAL_FILE}:"
cat "${TOTAL_FILE}"
rm robot-*.csv  # Clean up individual files
