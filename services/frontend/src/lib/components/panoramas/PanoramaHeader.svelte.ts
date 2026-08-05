import { updatePanorama } from '$lib/api/panoramas';
import { toast } from '@zerodevx/svelte-toast';

export class PanoramaHeaderLogic {
	async updateVisibility(id: string, visible: boolean) {
		const res = await updatePanorama(id, { visible });
		if (res.error) {
			toast.push(res.error.description);
		}
	}
}

export const componentState = new PanoramaHeaderLogic();
