import { useMapEvents } from "react-leaflet";

import { getMapTilerKey } from "@/utils/env";
import { useStore } from "@/store";

export const useLayerSelector = () => {
  const mapTilerKey = getMapTilerKey();
  const mapLayer = useStore((state) => state.mapLayer);
  const setMapLayer = useStore((state) => state.setMapLayer);
  const droneOverlay = useStore((state) => state.droneOverlay);
  const setDroneOverlay = useStore((state) => state.setDroneOverlay);

  useMapEvents({
    baselayerchange: (e) => {
      console.debug("Base layer changed to", e.name);
      setMapLayer(e.name);
    },

    overlayadd: (e) => {
      if (e.name === "Drone") {
        setDroneOverlay(true);
      }
    },

    overlayremove: (e) => {
      if (e.name === "Drone") {
        setDroneOverlay(false);
      }
    },
  });

  return {
    mapTilerKey,
    mapLayer,
    droneOverlay,
  };
};
