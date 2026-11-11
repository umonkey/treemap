import { restartPanorama } from '$lib/api/panoramas';
import type { IError } from '$lib/types';
import { goto } from '$app/navigation';
import type { PanoramaLayoutState } from '../layout.svelte.ts';

export class PageState {
	isSaving = $state<boolean>(false);
	error = $state<IError | undefined>(undefined);
	understandCost = $state<boolean>(false);
	deleteTemporaryFiles = $state<boolean>(false);

	submit = async (id: string, layout: PanoramaLayoutState) => {
		if (this.isSaving) return;
		this.isSaving = true;
		this.error = undefined;

		const res = await restartPanorama(id, this.deleteTemporaryFiles);

		this.isSaving = false;

		if (res.status === 200) {
			if (res.data) {
				layout.panorama = res.data;
			} else {
				await layout.reload(id);
			}
			goto(`/tools/panoramas/${id}`);
		} else {
			this.error = res.error;
		}
	};
}
