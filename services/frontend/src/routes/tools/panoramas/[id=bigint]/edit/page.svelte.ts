import { updatePanorama, type Panorama } from '$lib/api/panoramas';
import { roundOffset } from '$lib/utils/geo';
import type { IError } from '$lib/types';
import { goto } from '$app/navigation';
import type { PanoramaLayoutState } from '../layout.svelte.ts';

export class PageState {
	isSaving = $state<boolean>(false);
	error = $state<IError | undefined>(undefined);
	draft = $state<Panorama | undefined>(undefined);
	draftId: string | undefined = undefined;

	/** Seeds a rounded draft once per panorama, so a silent reload cannot clobber edits. */
	init = (panorama: Panorama | undefined) => {
		if (!panorama) return;
		if (this.draft && this.draftId === panorama.id) return;

		this.draftId = panorama.id;
		this.draft = {
			...panorama,
			lat_offset: roundOffset(panorama.lat_offset),
			lon_offset: roundOffset(panorama.lon_offset)
		};
	};

	save = async (layout: PanoramaLayoutState) => {
		const draft = this.draft;
		if (!draft || this.isSaving) return;
		this.isSaving = true;
		this.error = undefined;

		const lat_offset = roundOffset(draft.lat_offset);
		const lon_offset = roundOffset(draft.lon_offset);

		const res = await updatePanorama(draft.id, {
			title: draft.title,
			lat_offset,
			lon_offset
		});

		this.isSaving = false;

		if (res.status === 200) {
			if (res.data) {
				layout.panorama = res.data;
			} else {
				await layout.reload(draft.id);
			}
			goto(`/tools/panoramas/${draft.id}`);
		} else {
			this.error = res.error;
		}
	};
}
