import { useCallback, useState, useEffect } from "react";

import { ITreeInfo } from "./types";

export const useMarkers = () => {
  const center = [40.181389, 44.514444];
  const [markers, setMarkers] = useState([]);

  useEffect(() => {
    console.debug("useEffect");

    setMarkers((prev) => {
      const newMarkers = [...prev];

      newMarkers.push({
        lat: 40.181389,
        lon: 44.514444,
      });

      newMarkers.push({
        lat: 44.514444,
        lon: 40.181389,
      });

      console.debug('Added 2 markers.');

      return newMarkers;
    });
  }, []);

  return {
    center,
    markers,
  };
};
