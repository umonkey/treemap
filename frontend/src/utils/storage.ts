// Project imports.
import { IMapState, IUserInfo } from "@/types";
import { DEFAULT_MAP_CENTER, DEFAULT_MAP_ZOOM, HOME_PAGE_MAP_KEY, USER_INFO_KEY } from "@/utils/config";

export const DEFAULT_MAP_STATE = {
  center: DEFAULT_MAP_CENTER,
  zoom: DEFAULT_MAP_ZOOM,
} as IMapState;

const writeValue = <T>(key: string, value: T | null) => {
  try {
    const encoded = JSON.stringify(value);
    localStorage.setItem(key, encoded);
    console.debug(`[storage] Wrote ${key}`);
  } catch (e) {
    console.error(`[storage] Error writing ${key}`, e);
  }
};

const readValue = <T>(key: string): T | null => {
  try {
    const value = localStorage.getItem(key);

    if (value) {
      const decoded = JSON.parse(value);
      console.debug(`[storage] Read ${key}`);
      return decoded;
    }
  } catch (e) {
    console.error(`[storage] Error reading ${key}`, e);
  }

  return null;
};

export const getMapState = (): IMapState => {
  return readValue<IMapState>(HOME_PAGE_MAP_KEY) || DEFAULT_MAP_STATE;
};

export const setMapState = (value: IMapState) => {
  writeValue<IMapState>(HOME_PAGE_MAP_KEY, value);
}

export const getUserInfo = (): IUserInfo | null => {
  return readValue<IUserInfo>(USER_INFO_KEY);
};

export const setUserInfo = (value: IUserInfo | null) => {
  writeValue<IUserInfo>(USER_INFO_KEY, value);
};
