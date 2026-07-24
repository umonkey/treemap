import { getPanorama, updatePanorama, type Panorama } from '$lib/api/panoramas';
import type { IError } from '$lib/types';
import { goto } from '$app/navigation';

export class PageState {
	panorama = $state<Panorama | undefined>(undefined);
	isLoading = $state<boolean>(false);
	isSaving = $state<boolean>(false);
	error = $state<IError | undefined>(undefined);

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

	save = async () => {
		if (!this.panorama || this.isSaving) return;
		this.isSaving = true;
		this.error = undefined;

		const res = await updatePanorama(this.panorama.id, {
			title: this.panorama.title,
			visible: this.panorama.visible
		});

		this.isSaving = false;

		if (res.status === 200) {
			goto(`/admin/panoramas/${this.panorama.id}`);
		} else {
			this.error = res.error;
		}
	};
}
