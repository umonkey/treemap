import {
	getMapillarySequence,
	updateMapillarySequence,
	type MapillarySequenceDetail
} from '$lib/api/mapillary';
import type { IError } from '$lib/types';
import { goto } from '$app/navigation';

class PageState {
	sequence = $state<MapillarySequenceDetail | undefined>(undefined);
	isLoading = $state<boolean>(false);
	isSaving = $state<boolean>(false);
	error = $state<IError | undefined>(undefined);

	// Form fields
	title = $state<string>('');
	hidden = $state<boolean>(false);

	reload = async (id: string) => {
		this.isLoading = true;
		this.error = undefined;
		const res = await getMapillarySequence(id);
		this.isLoading = false;
		if (res.status === 200 && res.data) {
			this.sequence = res.data;
			this.title = res.data.title;
			this.hidden = res.data.hidden;
		} else {
			this.error = res.error;
		}
	};

	save = async (id: string) => {
		this.isSaving = true;
		this.error = undefined;
		const res = await updateMapillarySequence(id, {
			title: this.title,
			hidden: this.hidden
		});
		this.isSaving = false;
		if (res.status === 204) {
			goto('/admin/sequences');
		} else {
			this.error = res.error;
		}
	};
}

export const pageState = new PageState();
