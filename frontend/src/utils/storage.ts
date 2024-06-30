// Project imports.
import { IMapState, ILoginInfo } from "@/types";
import { DEFAULT_MAP_CENTER, DEFAULT_MAP_ZOOM, HOME_PAGE_MAP_KEY, USER_INFO_KEY, MAP_LAYER_KEY } from "@/utils/config";

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

export const getLoginInfo = (): ILoginInfo | null => {
  return readValue<ILoginInfo>(USER_INFO_KEY);
};

export const setLoginInfo = (value: ILoginInfo | null) => {
  writeValue<ILoginInfo>(USER_INFO_KEY, value);
};

export const getMapLayer = (): string => {
  return readValue<string>(MAP_LAYER_KEY) || "OpenStreetMap";
};

export const setMapLayer = (value: string) => {
  writeValue<string>(MAP_LAYER_KEY, value);
};

export const getDebug = (): boolean => {
  return readValue<boolean>("debug") || false;
}

export const setDebug = (value: boolean) => {
  writeValue<boolean>("debug", value);
}
