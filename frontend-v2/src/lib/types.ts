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
	files: ITreeFile[];
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
	lat?: number;
	lon?: number;
	address?: string;
	species?: string;
	notes?: string;
	height?: number;
	circumference?: number;
	diameter?: number;
	state?: string;
}

export interface IMeResponse {
	name: string;
	picture: string;
}

export interface ISpecies {
	name: string;
	local: string;
}

export interface IComment {
	id: string;
	added_at: number;
	added_by: string;
	message: string;
}
