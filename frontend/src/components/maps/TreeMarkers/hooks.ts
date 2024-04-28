import { useCallback, useEffect, useRef, useState } from "react";

import { useSearchQuery } from "@/hooks";
import { IBounds, ITreeInfoMap } from "@/types";
import { treeMapService } from "@/services/api";
import { mainBus } from "@/bus";

const RELOAD_DELAY = 100;

export const useMarkers = () => {
  // This is all markers that we ever loaded, indexed by key.
  const [map, setMap] = useState<ITreeInfoMap>({});

  // Last displayed bounds, tracked to request the API.
  const [bounds, setBounds] = useState<number[]>([]);

  // Search string.
  const { searchQuery } = useSearchQuery();

  const timeoutId = useRef<ReturnType<typeof setTimeout>|null>(null);

  const handleSearchQueryChange = useCallback(() => {
    console.debug("Search query changed, emptying the map.");
    setMap({});
  }, []);

  // Listen for the search query change.
  useEffect(() => {
    mainBus.on("before_search", handleSearchQueryChange);
    return () => mainBus.off("before_search", handleSearchQueryChange);
  });

  /**
   * Reload markers on map move or zoom.
   */
  const reload = useCallback((newBounds: IBounds) => {
    const updated = [
      newBounds.north,
      newBounds.east,
      newBounds.south,
      newBounds.west,
    ];

    setBounds(updated);
  }, []);

  const fetchMarkers = useCallback(async () => {
    const res = await treeMapService.getMarkers({
      north: bounds[0],
      east: bounds[1],
      south: bounds[2],
      west: bounds[3],
      search: searchQuery,
    });

    timeoutId.current = null;

    console.debug(`Received ${res.length} markers from the API.`);

    let added = 0;
    const updated = { ...map };

    for (const marker of res) {
      if (!(marker.id in updated)) {
        updated[marker.id] = marker;
        added++;
      }
    }

    if (added) {
      console.debug(`Added ${added} new markers.`);
      setMap(updated);
    }
  }, [bounds, map, searchQuery]);

  const handleBoundsChange = (bounds: IBounds) => {
    reload(bounds);
  };

  useEffect(() => {
    if (bounds.length !== 4) {
      return;
    }

    if (timeoutId.current) {
      clearTimeout(timeoutId.current);
    }

    timeoutId.current = setTimeout(() => {
      fetchMarkers();
    }, RELOAD_DELAY);
  }, [bounds, fetchMarkers]);

  return {
    markers: Object.values(map),
    handleBoundsChange,
  };
};
