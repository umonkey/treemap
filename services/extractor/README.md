# Street View Image Extractor Service

This service creates series of equirectangular JPEG images for a custom street-view implementation.  It extracts still frames from a 360 video, at fixed intervals (3 meters currently), then uses OpenSfM to increase coordinate accuracy, then uploads the resulting imagery to S3.  The results are still images with a JSON file listing them, ready to be used by the app.
