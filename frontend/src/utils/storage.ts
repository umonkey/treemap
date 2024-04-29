// Project imports.
import { IMapState } from "@/types";
import { DEFAULT_MAP_CENTER, DEFAULT_MAP_ZOOM, HOME_PAGE_MAP_KEY } from "@/utils/config";

export const DEFAULT_MAP_STATE = {
  center: DEFAULT_MAP_CENTER,
  zoom: DEFAULT_MAP_ZOOM,
} as IMapState;

export const getMapState = (): IMapState => {
  const key = HOME_PAGE_MAP_KEY;

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

export const setMapState = (value: IMapState) => {
  const key = HOME_PAGE_MAP_KEY;

  try {
    const encoded = JSON.stringify(value);
    localStorage.setItem(key, encoded);
    console.debug(`Map state updated in local storage, key=${key}`);
  } catch (e) {
    console.error(`Error updating map state in local storage, key=${key}.`, e);
  }
}
