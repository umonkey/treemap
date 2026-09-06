# Street View Image Extractor Service

This service creates series of equirectangular JPEG images for a custom street-view implementation.  It extracts still frames from a 360 video, at fixed intervals (3 meters currently), then uses OpenSfM to increase coordinate accuracy, then uploads the resulting imagery to S3.  The results are still images with a JSON file listing them, ready to be used by the app.

## Things to try

### Combine Order Neighbors with BoW Matching

You do not need slow, brute-force all-to-all matching. Instead, keep your sequential window for smooth local tracking and add Bag-of-Words (BoW) / visual words to automatically catch loop closures anywhere in the sequence:

```yaml
# 1. Keep fast, robust local tracking
matching_order_neighbors: 10

# 2. Automatically detect loop closures (even if you don't know where/if they occur)
matching_bow_neighbors: 15
bow_words_to_query: 50
```

How this works:

- matching_order_neighbors: 10 handles your consecutive walking track.
- matching_bow_neighbors: 15 uses an inverted index of image features (visual vocabulary) to query the most visually similar frames across the entire dataset. If frame 500 looks at the same spot as frame 0, it will be selected, matched, and the loop will be closed rigidly in Bundle Adjustment.
- If the path does not loop, BoW simply fails to find strong global pairs, and processing continues without wasting $O(N^2)$ comparisons.


### Force Pure Photogrammetric Rigidity First (Zero GPS)

- Walk in complete loops or cross-hatching paths (lawn crossings) whenever possible.
- Use matching_bow_neighbors: 20 alongside matching_order_neighbors: 10 so 360-degree views across the courtyard lock opposite sides together. Full 360 equirectangular frames give massive geometric leverage across open courtyards that standard directional cameras cannot match.
- Let OpenSfM build an internally rigid, non-deformed local coordinate space.


### Second pass

1. Run OpenSfM
2. Use the algorithm to match the GPS track part to synchronize automatically
3. Re-run bundle adjustment with the GPS data as a hint, using `matching_bow_neighbors: 30`
