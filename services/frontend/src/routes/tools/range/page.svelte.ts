import { routes, goto } from '$lib/routes';
import type { ILatLng } from '$lib/types';
import { rangeStore } from './store.svelte';
import type { IGcpWithRadius } from './MapPreview.svelte.ts';

export class RangeSetupState {
	gcps = $derived(rangeStore.gcps);

	setGcp = (index: number, val: ILatLng) => {
		rangeStore.setGcp(index, val);
	};

	clearGcp = (index: number) => {
		rangeStore.clearGcp(index);
	};

	validCount = $derived(rangeStore.validCount);
	canContinue = $derived(rangeStore.canContinue);

	gcpsWithLabels = $derived<IGcpWithRadius[]>(
		this.gcps.map((g, i) => ({
			lat: g ? g.lat : Number.NaN,
			lng: g ? g.lng : Number.NaN,
			radius: 0,
			index: i + 1,
			label: String.fromCharCode(65 + i)
		}))
	);

	handleContinue = () => {
		if (this.canContinue) {
			goto(routes.toolsRangeEnter());
		}
	};
}
