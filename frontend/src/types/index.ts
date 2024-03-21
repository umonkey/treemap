export interface IBounds {
  north: number;
  east: number;
  south: number;
  west: number;
}

export interface ILatLng {
  lat: number;
  lon: number;
}

export interface ITreeInfo {
  id: number;
  lat: number;
  lon: number;
  name: string;
}

export enum SideBarMode {
  DEFAULT = "default",
  ADD_TREE = "add_tree",
}
