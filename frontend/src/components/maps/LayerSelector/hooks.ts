import { useMapEvents } from "react-leaflet";

import { getMapTilerKey } from "@/utils/env";
import { useStore } from "@/store";

export const useLayerSelector = () => {
  const mapTilerKey = getMapTilerKey();
  const mapLayer = useStore((state) => state.mapLayer);
  const setMapLayer = useStore((state) => state.setMapLayer);

  useMapEvents({
    baselayerchange: (e) => {
      console.debug("Base layer changed to", e.name);
      setMapLayer(e.name);
    },
  });

  return {
    mapTilerKey,
    mapLayer,
  };
};
