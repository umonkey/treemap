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
  osm_id: number | null;
  species: string;
  notes: string | null;
  height: number | null;
  circumference: number | null;
  diameter: number | null;
  state: string;
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
  osm_id: number | null;
  species: string;
  notes: string | null;
  height: number | null;
  circumference: number | null;
  diameter: number | null;
  state: string;
  updated_at: number;
  thumbnail_id: string | null;
  files?: IFileInfo[];
}

export interface IAddTreesRequest {
  points: ILatLng[];
  species: string;
  notes: string | null;
  height: number | null;
  circumference: number | null;
  diameter: number | null;
  state: string;
}

export interface IUpdateTreeRequest {
  lat: number;
  lon: number;
  species: string;
  notes: string | null;
  height: number | null;
  circumference: number | null;
  diameter: number | null;
  state: string;
}

export interface IAddRowRequest {
  lat1: number;
  lon1: number;
  lat2: number;
  lon2: number;
  species: string;
  number: number;
  notes: string | null;
  height: number | null;
  circumference: number | null;
  diameter: number | null;
  state: string;
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

export interface IMarkerClickEvent {
  id: string;
  position: ILatLng;
}

export interface ITreeDefaults {
  species: string | null;
  notes: string | null;
  height: number | null;
  circumference: number | null;
  diameter: number | null;
  state: string;
}

export interface IFileUploadRequest {
  tree: string;
  file: File;
}

export interface IFileUploadResponse {
  id: string;
}

export interface IFileStatusResponse {
  ready: boolean;
}

export interface IFileReadyEvent {
  file_id: string;
  tree_id: string;
}

export interface ITreeStats {
  count: number;
}

export enum SideBarMode {
  DEFAULT = "default",
  ADD_TREE = "add_tree",
  ADD_TREE_DESCRIPTION = "add_tree_description",
}
