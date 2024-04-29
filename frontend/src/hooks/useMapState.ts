/**
 * A hook to save and restore map position (center and zoom).
 */

import { useMemo } from "react";

import { IMapState } from "@/types";
import { getMapState, setMapState } from "@/utils";

export const useMapState = () => {
  const state = useMemo(() => getMapState(), []);

  const change = (value: IMapState) => {
    setMapState(value);
  };

  return {
    mapState: state,
    setMapState: change,
  };
};
