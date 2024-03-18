import { useCallback, useState, useEffect } from "react";

import { ITreeInfo } from "./types";

export const useMarkers = () => {
  const center = [40.181389, 44.514444];
  const [markers, setMarkers] = useState([]);

  useEffect(() => {
    markers.push({
      lat: 40.181389,
      lon: 44.514444,
    });

    markers.push({
      lat: 44.514444,
      lon: 40.181389,
    });

    console.debug('Added 2 markers.');
  }, []);

  return {
    center,
    markers,
  };
};
