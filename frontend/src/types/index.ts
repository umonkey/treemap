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
  id: string;
  lat: number;
  lon: number;
  name: string;
  height: number | null;
  circumference: number | null;
  diameter: number | null;
  state: string | null;
  updated_at: number;
}

export interface IUserInfo {
  token: string;
  name: string;
  picture: string;
}

export interface ITreeDetails {
  id: string;
  lat: number;
  lon: number;
  name: string;
  height: number | null;
  circumference: number | null;
  diameter: number | null;
  state: string | null;
  updated_at: number;
}

export interface IAddTreeRequest {
  lat: number;
  lon: number;
  name: string;
  height: number | null;
  circumference: number | null;
  diameter: number | null;
  state: string;
}

export enum SideBarMode {
  DEFAULT = "default",
  ADD_TREE = "add_tree",
  ADD_TREE_DESCRIPTION = "add_tree_description",
}
