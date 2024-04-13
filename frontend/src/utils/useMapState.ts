/**
 * A hook to save and restore map position (center and zoom).
 */

import { useState } from "react";

import { IMapState } from "@/types";
import { DEFAULT_MAP_CENTER, DEFAULT_MAP_ZOOM } from "./config";

export const DEFAULT_MAP_STATE = {
  center: DEFAULT_MAP_CENTER,
  zoom: DEFAULT_MAP_ZOOM,
} as IMapState;

const extract = (key: string): IMapState => {
  try {
    const value = localStorage.getItem(key);

    if (value) {
      const decoded = JSON.parse(value);
      console.debug(`Map state read from storage, key=${key}`);
      return decoded;
    }
  } catch (e) {
    console.error(`Error reading map state from local storage key=${key}.`, e);
    return DEFAULT_MAP_STATE;
  }

  console.debug(`Map state not found in local storage, key=${key}`);
  return DEFAULT_MAP_STATE;
};

const update = (key: string, value: IMapState) => {
  try {
    const encoded = JSON.stringify(value);
    localStorage.setItem(key, encoded);
    console.debug(`Map state updated in local storage, key=${key}`);
  } catch (e) {
    console.error(`Error updating map state in local storage, key=${key}.`, e);
  }
}

export const useMapState = (key: string) => {
  const [state, setState] = useState<IMapState>(extract(key));

  const setMapState = (value: IMapState) => {
    update(key, value);
    setState(value);
  }

  return {
    mapState: state,
    setMapState,
  };
};
