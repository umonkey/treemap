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
  thumbnail_id: string | null;
}

export interface ITreeInfoMap {
  [id: string]: ITreeInfo;
}

export interface IUserInfo {
  token: string;
  name: string;
  picture: string;
}

export interface IFileInfo {
  id: string;
  small_id: string;
  large_id: string;
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
  thumbnail_id: string | null;
  files?: IFileInfo[];
}

export interface IAddTreeRequest {
  lat: number;
  lon: number;
  name: string;
  height: number | null;
  circumference: number | null;
  diameter: number | null;
  state: string;
  notes: string | null;
}

/**
 * This is used to save and restore the state of a map view.
 */
export interface IMapState {
  center: ILatLng;
  zoom: number;
}

/**
 * This is used by the map component to report changes in the
 * view.  Not necessarily all of this is used for remembering
 * the position, but for stuff like loading markers.
 */
export interface IMapView {
  center: ILatLng;
  zoom: number;
  bounds: IBounds;
}

export interface IApiError {
  status: number;
  code: string;
  message: string;
}

export interface IUploadTicket {
  id: string;
  upload_url: string;
}

export interface IGalleryImage {
  small: string;
  large: string;
}

export interface IComment {
  id: string;
  added_at: number;
  message: string;
}

export interface ISpecies {
  name: string;
  local: string;
}

export enum SideBarMode {
  DEFAULT = "default",
  ADD_TREE = "add_tree",
  ADD_TREE_DESCRIPTION = "add_tree_description",
}
