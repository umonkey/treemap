export interface MapRay {
	lat: number;
	lng: number;
	angle: number;
	length?: number;
}

class MapRaysStore {
	rays = $state<MapRay[]>([]);
}

export const mapRaysStore = new MapRaysStore();
