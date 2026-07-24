import { getPanoramaTrackGeoJSON } from '$lib/api/panoramas';
import { config } from '$lib/env';
import { locale } from '$lib/locale';
import type { FeatureCollection } from 'geojson';

class TrackPreviewState {
	trackData = $state<FeatureCollection | undefined>(undefined);
	loading = $state<boolean>(false);
	error = $state<string | undefined>(undefined);

	layer = `https://api.maptiler.com/maps/positron/style.json?key=${config.mapTilerKey}&language=${locale.lang}`;

	constructor() {
		// Pure constructor
	}

	reload = async (panoramaId: string) => {
		this.loading = true;
		this.error = undefined;
		const res = await getPanoramaTrackGeoJSON(panoramaId);
		this.loading = false;
		if (res.status === 200 && res.data) {
			this.trackData = res.data as FeatureCollection;
		} else {
			this.error = res.error?.description || 'Failed to load track data';
		}
	};
}

export const componentState = new TrackPreviewState();
