import { useState, useEffect } from "react";

import { ITreeInfo } from "./types";
import { treeMapService } from "../../services/api";

export const useMarkers = () => {
  const center = [40.181389, 44.514444];
  const [markers, setMarkers] = useState<ITreeInfo>([]);

  useEffect(() => {
    (async () => {
      const res = await treeMapService.getMarkers();
      console.debug("MARKERS", res);

      setMarkers(res);
    })();

    // Fix markers multiplying on every re-render.
    return () => {
      setMarkers([]);
    };
  }, []);

  console.debug(`Have ${markers.length} markers.`);

  return {
    center,
    markers,
  };
};
