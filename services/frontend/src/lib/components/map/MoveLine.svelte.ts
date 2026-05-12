import { mapState } from './MapLibre.svelte.ts';
import { moveOriginState } from '$lib/stores/moveOriginState.svelte.ts';

class MoveLineLogic {
	line = $derived.by(() => {
		const origin = moveOriginState.origin;
		const center = mapState.center;

		if (!origin || !center) {
			return undefined;
		}

		return {
			type: 'Feature' as const,
			properties: {},
			geometry: {
				type: 'LineString' as const,
				coordinates: [
					[origin.lng, origin.lat],
					[center.lng, center.lat]
				]
			}
		};
	});
}

export const componentState = new MoveLineLogic();
