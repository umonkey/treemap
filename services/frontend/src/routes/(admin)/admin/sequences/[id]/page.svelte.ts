import { getMapillarySequence, type MapillarySequenceDetail } from '$lib/api/mapillary';
import type { IError } from '$lib/types';

class PageState {
	sequence = $state<MapillarySequenceDetail | undefined>(undefined);
	isLoading = $state<boolean>(false);
	error = $state<IError | undefined>(undefined);

	reload = async (id: string) => {
		this.isLoading = true;
		this.error = undefined;
		const res = await getMapillarySequence(id);
		this.isLoading = false;
		if (res.status === 200 && res.data) {
			this.sequence = res.data;
		} else {
			this.error = res.error;
		}
	};
}

export const pageState = new PageState();
