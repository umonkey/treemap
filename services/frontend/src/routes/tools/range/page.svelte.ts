import { routes, goto } from '$lib/routes';
import type { ILatLng } from '$lib/types';
import { rangeStore } from './store.svelte';

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

	handleContinue = () => {
		if (this.canContinue) {
			goto(routes.toolsRangeEnter());
		}
	};
}
