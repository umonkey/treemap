#!/bin/bash
# process-sfm.sh - Master Turn-Aware OpenSfM pipeline for 8K Panoramas
set -e

DOCKER_IMAGE="mapillary/opensfm"

# 1. Verification
if [ ! -d "images" ]; then
    echo "Error: 'images' directory not found."
    exit 1
fi

# 2. Clean Start
echo "Cleaning intermediate results..."
# We keep features/ and matches/ if they exist to save time, but reset metadata
rm -rf exif camera_models.json camera_models_overrides.json reports reconstruction.json

# 3. Setup Camera Overrides (Master Scale)
echo '{"all": {"projection_type": "spherical", "width": 7680, "height": 3840}}' > camera_models_overrides.json

# Function to run opensfm commands in docker
run_sfm() {
    local cmd=$1
    echo "--------------------------------------------------------"
    echo "Running: $cmd"
    echo "--------------------------------------------------------"
    docker run --rm -v "$(pwd):/data" -w /data "$DOCKER_IMAGE" /source/OpenSfM/bin/opensfm "$cmd" .
}

# 4. Extract Metadata (Raw Pass)
run_sfm extract_metadata 2>&1 | tee extract_metadata.log

# 5. Inject Master Settings (Smoothed GPS, DOP, Spherical)
# This script now overwrites exif/*.exif files directly
echo "Applying Turn-Aware GPS Smoothing and metadata injection..."
bin/smooth-gps

# 6. Core SfM Pipeline
run_sfm detect_features 2>&1 | tee detect_features.log
run_sfm match_features 2>&1 | tee match_features.log
run_sfm create_tracks 2>&1 | tee create_tracks.log
run_sfm reconstruct 2>&1 | tee reconstruct.log

# 7. Post-processing
echo "Pruning outlier points (threshold: 500m)..."
python3 prune_points.py
run_sfm export_ply 2>&1 | tee export_ply.log

# 8. Quality Verification
bin/verify-error 2>&1 | tee export_ply.log

# 9. Final Output Generation
echo "Writing corrected images to 'output/' folder..."
rm -rf output && mkdir -p output
bin/write-output

# 10. Show Changes
bin/show-changes 2>&1 | tee show_changes.log

echo "--------------------------------------------------------"
echo "Processing Complete!"
echo "Check the 'output/' folder for your corrected images."
echo "--------------------------------------------------------"
