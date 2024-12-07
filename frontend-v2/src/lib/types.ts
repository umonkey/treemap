export interface ITreeFile {
	id: string;
	small_id: string;
	large_id: string;
	added_at: number | null;
	added_by: string | null;
}

export interface ITree {
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
	added_at: number;
	updated_at: number;
	added_by: string;
	files: ITreeFile[];
}

export interface IMarkers {
	trees: ITree[];
}

export interface IStats {
	count: number;
}
