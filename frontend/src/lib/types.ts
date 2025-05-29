export type { IError, IResponse } from './types_response';

export interface ILatLng {
	lat: number;
	lng: number;
}

export type MountFn = (fn: () => void) => void;
export type DestroyFn = (fn: () => void) => void;

export interface ITreeFile {
	id: string;
	small_id: string;
	large_id: string;
	added_at?: number;
	added_by?: string;
}

export interface IUser {
	id: string;
	name: string;
	picture: string;
}

export interface ITree {
	id: string;
	lat: number;
	lon: number;
	address: string | null;
	osm_id: number | null;
	species: string;
	notes: string | null;
	height: number | null;
	circumference: number | null;
	diameter: number | null;
	state: string;
	added_at: number;
	updated_at: number;
	added_by: string;
	year: number | null;
	replaces: string | null;
	replaced_by: string | null;
	files: ITreeFile[];
}

export interface ISingleTree extends ITree {
	users: IUser[];
}

export interface IMarkers {
	trees: ITree[];
}

export interface IStats {
	count: number;
}

export interface ILoginResponse {
	token: string;
	name: string;
	picture: string;
}

export interface ITreeUpdatePayload {
	lat?: number | null;
	lon?: number | null;
	address?: string | null;
	species?: string | null;
	notes?: string | null;
	height?: number | null;
	circumference?: number | null;
	diameter?: number | null;
	state?: string | null;
	year?: number | null;
}

export interface IMeResponse {
	id: string;
	name: string;
	email: string;
	picture: string;
	trees_count: number;
	comments_count: number;
	updates_count: number;
	files_count: number;
}

export interface ISpecies {
	name: string;
	local: string;
}

export interface IComment {
	id: string;
	tree_id: number;
	added_at: number;
	added_by: string;
	message: string;
}

export interface ICommentList {
	comments: IComment[];
	users: IUser[];
	trees: ITree[];
}

export interface IChange {
	id: string;
	tree_id: string;
	name: string;
	value: string;
	added_at: number;
	added_by: string;
}

export interface IChangeList {
	props: IChange[];
	users: IUser[];
}

export interface ILatLon {
	lat: number;
	lon: number;
}

export interface IAddTreesRequest {
	points: ILatLon[];
	species: string;
	notes: string | null;
	height: number | null;
	circumference: number | null;
	diameter: number | null;
	year: number | null;
	state: string | null;
	files: string[];
}

export interface IReplaceTreeRequest {
	species: string;
	notes: string | null;
	height: number | null;
	circumference: number | null;
	diameter: number | null;
	year: number | null;
	state: string | null;
	files: string[];
}

export interface ITreeList {
	trees: ITree[];
	users: IUser[];
}

export interface ISpeciesStats {
	species: string;
	count: number;
}

export interface IStreetStats {
	address: string;
	count: number;
}

export interface IStateStats {
	state: string;
	count: number;
}

export interface IMyPosition {
	lat: number;
	lng: number;
	accuracy: number;
}

export interface ILike {
	tree_id: string;
	user_id: string;
}

export interface ITreeDefaults {
	species: string | null;
	notes: string | null;
	height: number | null;
	circumference: number | null;
	diameter: number | null;
	state: string | null;
}

export interface ILikeList {
	likes: ILike[];
	users: IUser[];
	trees: ITree[];
}

/**
 * This is the raw error object that we receive from the API.
 */
export interface IRawError {
	error: {
		code: string;
		description: string;
	};
}

export interface IGalleryItem {
	small: string;
	large: string;
	label: string;
}

/**
 * File upload response.
 * The id is used for creating new photos.
 */
export interface IUploadResponse {
	id: string;
}
