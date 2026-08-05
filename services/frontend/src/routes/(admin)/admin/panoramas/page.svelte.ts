import { getPanoramas, updatePanorama, type Panorama } from '$lib/api/panoramas';
import type { IError } from '$lib/types';
import { showError } from '$lib/errors';

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

	updateVisibility = async (id: string, visible: boolean) => {
		const res = await updatePanorama(id, { visible });
		if (res.status === 200 && res.data) {
			const index = this.panoramas.findIndex((p) => p.id === id);
			if (index !== -1) {
				this.panoramas[index].visible = visible;
			}
		} else {
			showError(res.error?.description || 'Failed to update visibility');
			await this.reload();
		}
	};
}

export const pageState = new PageState();
