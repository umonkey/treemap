export interface ITree {
	id: string;
	lat: number;
	lon: number;
	osm_id: number | null;
	species: string;
	notes: string | null;
}
