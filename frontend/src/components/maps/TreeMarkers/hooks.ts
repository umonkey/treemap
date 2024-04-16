import { useCallback, useEffect, useRef, useState } from "react";

import { IBounds, ITreeInfoMap } from "@/types";
import { treeMapService } from "@/services/api";

const RELOAD_DELAY = 100;

export const useMarkers = () => {
  // This is all markers that we ever loaded, indexed by key.
  const [map, setMap] = useState<ITreeInfoMap>({});

  // Last displayed bounds, tracked to request the API.
  const [bounds, setBounds] = useState<number[]>([]);

  const timeoutId = useRef<ReturnType<typeof setTimeout>|null>(null);

  /**
   * Reload markers on map move or zoom.
   */
  const reload = (newBounds: IBounds) => {
    const updated = [
      newBounds.north,
      newBounds.east,
      newBounds.south,
      newBounds.west,
    ];

    setBounds(updated);
  };

  const fetchMarkers = useCallback(async () => {
    const res = await treeMapService.getMarkers({
      north: bounds[0],
      east: bounds[1],
      south: bounds[2],
      west: bounds[3],
    });

    timeoutId.current = null;

    console.debug(`Received ${res.length} markers from the API.`);

    setMap((prev) => {
      const updated = { ...prev };

      for (const marker of res) {
        updated[marker.id] = marker;
      }

      return updated;
    });
  }, [bounds]);

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
    reload,
  };
};
