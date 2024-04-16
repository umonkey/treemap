import { useCallback, useEffect, useRef, useState } from "react";

import { IBounds, ITreeInfo } from "@/types";
import { treeMapService } from "@/services/api";

const RELOAD_DELAY = 100;

export const useMarkers = () => {
  const [markers, setMarkers] = useState<ITreeInfo[]>([]);
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

    setMarkers(res);
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
    markers,
    reload,
  };
};
