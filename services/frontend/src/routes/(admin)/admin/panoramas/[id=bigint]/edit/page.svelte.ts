import { getPanorama, updatePanorama, type Panorama } from '$lib/api/panoramas';
import { roundOffset } from '$lib/utils/geo';
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
			this.panorama = {
				...res.data,
				lat_offset: roundOffset(res.data.lat_offset),
				lon_offset: roundOffset(res.data.lon_offset)
			};
		} else {
			this.error = res.error;
		}
	};

	save = async () => {
		if (!this.panorama || this.isSaving) return;
		this.isSaving = true;
		this.error = undefined;

		const lat_offset = roundOffset(this.panorama.lat_offset);
		const lon_offset = roundOffset(this.panorama.lon_offset);

		const res = await updatePanorama(this.panorama.id, {
			title: this.panorama.title,
			lat_offset,
			lon_offset
		});

		this.isSaving = false;

		if (res.status === 200) {
			goto(`/admin/panoramas/${this.panorama.id}`);
		} else {
			this.error = res.error;
		}
	};
}
