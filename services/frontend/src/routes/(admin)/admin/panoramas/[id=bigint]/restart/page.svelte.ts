import { getPanorama, restartPanorama, type Panorama } from '$lib/api/panoramas';
import type { IError } from '$lib/types';
import { goto } from '$app/navigation';

export class PageState {
	panorama = $state<Panorama | undefined>(undefined);
	isLoading = $state<boolean>(false);
	isSaving = $state<boolean>(false);
	error = $state<IError | undefined>(undefined);
	understandCost = $state<boolean>(false);

	reload = async (id: string) => {
		this.isLoading = true;
		this.error = undefined;
		const res = await getPanorama(id);
		this.isLoading = false;
		if (res.status === 200 && res.data) {
			this.panorama = res.data;
		} else {
			this.error = res.error;
		}
	};

	submit = async (id: string) => {
		if (this.isSaving) return;
		this.isSaving = true;
		this.error = undefined;

		const res = await restartPanorama(id);

		this.isSaving = false;

		if (res.status === 200) {
			goto(`/admin/panoramas/${id}`);
		} else {
			this.error = res.error;
		}
	};
}
