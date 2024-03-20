import { useState, useEffect } from "react";

import { treeMapService } from "../../../services/api";
import { IBounds, ILatLng, ITreeInfo } from "../../../types";

const coordsChanged = (a: number[], b: number[]): boolean => {
  const _a = a.join(";");
  const _b = b.join(";");
  return _a !== _b;
};

export const useMarkers = () => {
  const center: ILatLng = {
    lat: 40.181389,
    lon: 44.514444,
  };

  const [markers, setMarkers] = useState<ITreeInfo[]>([]);
  const [bounds, setBounds] = useState<number[]>([]);

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

    if (coordsChanged(bounds, updated)) {
      setBounds(updated);
    }
  };

  useEffect(() => {
    if (bounds.length !== 4) {
      return;
    }

    (async () => {
      const res = await treeMapService.getMarkers({
        north: bounds[0],
        east: bounds[1],
        south: bounds[2],
        west: bounds[3],
      });

      console.debug(`Received ${res.length} markers.`);

      setMarkers(res);
    })();
  }, [bounds]);

  return {
    center,
    markers,
    reload,
  };
};
