import { getMapillarySequences, type MapillarySequenceSummary } from '$lib/api/mapillary';
import type { IError } from '$lib/types';

class PageState {
	sequences = $state<MapillarySequenceSummary[]>([]);
	isLoading = $state<boolean>(false);
	error = $state<IError | undefined>(undefined);

	reload = async () => {
		this.isLoading = true;
		this.error = undefined;
		const res = await getMapillarySequences();
		this.isLoading = false;
		if (res.status === 200 && res.data) {
			this.sequences = res.data;
		} else {
			this.error = res.error;
		}
	};
}

export const pageState = new PageState();
