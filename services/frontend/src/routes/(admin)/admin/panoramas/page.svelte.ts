import { getPanoramas, type Panorama } from '$lib/api/panoramas';
import type { IError } from '$lib/types';

class PageState {
	panoramas = $state<Panorama[]>([]);
	isLoading = $state<boolean>(false);
	error = $state<IError | undefined>(undefined);

	reload = async () => {
		this.isLoading = true;
		this.error = undefined;
		const res = await getPanoramas();
		this.isLoading = false;
		if (res.status === 200 && res.data) {
			this.panoramas = res.data;
		} else {
			this.error = res.error;
		}
	};
}

export const pageState = new PageState();
