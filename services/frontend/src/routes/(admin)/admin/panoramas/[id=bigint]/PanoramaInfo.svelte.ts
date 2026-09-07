import { deletePanoramaHints, type Panorama } from '$lib/api/panoramas';
import { panoBus } from '$lib/buses/panoBus';
import { storage_cost } from '$lib/utils/files';
import type { IError } from '$lib/types';

export class PanoramaInfoLogic {
	isClearingHints = $state(false);
	error = $state<IError | undefined>(undefined);

	clearHints = async (id: string, panorama: Panorama) => {
		if (
			!confirm(
				'Are you sure you want to delete all hints for this panorama? This action cannot be undone.'
			)
		) {
			return;
		}
		this.isClearingHints = true;
		this.error = undefined;
		const res = await deletePanoramaHints(id);
		this.isClearingHints = false;
		if (res.status === 204 || res.status === 200) {
			panorama.hints_count = 0;
			panoBus.emit('reload');
		} else {
			this.error = res.error;
		}
	};

	formatFileSizeGb = (fileSize?: number | null): string => {
		const size = fileSize ?? 0;
		const gb = Math.round(size / (1024 * 1024 * 1024));
		const cost = storage_cost(size);
		return `${gb} GB ≈ ${cost}/mo`;
	};

	formatProcessingTime = (seconds?: number | null): string => {
		const s = seconds ?? 0;
		const hours = s / 3600;
		const cost = hours * 0.4;
		return `${hours.toFixed(1)} hours ≈ $${cost.toFixed(2)}`;
	};
}
