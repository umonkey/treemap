import { getPanorama, exportPanorama, type Panorama } from '$lib/api/panoramas';
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

	exportData = async (id: string) => {
		this.isLoading = true;
		this.error = undefined;
		const res = await exportPanorama(id);
		this.isLoading = false;
		if (res.status === 200 && res.data) {
			const jsonString = JSON.stringify(res.data, null, 2);
			const blob = new Blob([jsonString], { type: 'application/json' });
			const url = URL.createObjectURL(blob);
			const a = document.createElement('a');
			a.href = url;
			a.download = `panorama-${id}.json`;
			document.body.appendChild(a);
			a.click();
			document.body.removeChild(a);
			URL.revokeObjectURL(url);
		} else {
			this.error = res.error;
		}
	};
}
