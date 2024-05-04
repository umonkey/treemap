import { getMapState } from "@/utils/storage";

export const useMapWithMarker = () => {
  const state = getMapState();

  return {
    zoom: state.zoom,
  };
};
