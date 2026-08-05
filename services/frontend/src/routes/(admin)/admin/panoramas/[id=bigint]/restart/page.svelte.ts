import { getPanorama, restartPanorama, type Panorama } from '$lib/api/panoramas';
import type { IError } from '$lib/types';
import { goto } from '$app/navigation';

export class PageState {
	panorama = $state<Panorama | undefined>(undefined);
	isLoading = $state<boolean>(false);
	isSaving = $state<boolean>(false);
	error = $state<IError | undefined>(undefined);
	eraseResults = $state<boolean>(false);
	eraseTempFiles = $state<boolean>(true);

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

		const res = await restartPanorama(id, {
			erase_results: this.eraseResults,
			erase_temp_files: this.eraseTempFiles
		});

		this.isSaving = false;

		if (res.status === 200) {
			goto(`/admin/panoramas/${id}`);
		} else {
			this.error = res.error;
		}
	};
}
