import { getPanorama, type Panorama } from '$lib/api/panoramas';
import type { IError } from '$lib/types';

export class PageState {
	panorama = $state<Panorama | undefined>(undefined);
	isLoading = $state<boolean>(false);
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
}
