import { getPanorama, type Panorama } from '$lib/api/panoramas';
import type { IError } from '$lib/types';
import { onPageFocus } from '$lib/utils/onPageFocus';

export const PANORAMA_CONTEXT_KEY = Symbol('panorama');

export class PanoramaLayoutState {
	panorama = $state<Panorama | undefined>(undefined);
	isLoading = $state<boolean>(false);
	error = $state<IError | undefined>(undefined);
	currentId: string | undefined = undefined;
	private pollInterval: ReturnType<typeof setInterval> | null = null;

	reload = async (id: string, options?: { silent?: boolean }) => {
		this.stopPolling();
		this.currentId = id;
		if (!options?.silent) {
			this.isLoading = true;
		}
		this.error = undefined;
		const res = await getPanorama(id);
		if (!options?.silent) {
			this.isLoading = false;
		}
		if (res.status === 200 && res.data) {
			this.panorama = res.data;
		} else {
			this.error = res.error;
		}
	};

	public setup = () => {
		return onPageFocus(() => {
			if (this.currentId && !this.isLoading) {
				this.reload(this.currentId, { silent: true });
			}
		});
	};

	startPolling = (id: string) => {
		if (this.pollInterval) return;
		this.pollInterval = setInterval(async () => {
			const res = await getPanorama(id);
			if (res.status === 200 && res.data) {
				this.panorama = res.data;
				if (this.panorama.status === 'SUCCESS' || this.panorama.status === 'FAILURE') {
					this.stopPolling();
				}
			} else if (res.error) {
				this.error = res.error;
			}
		}, 10000);
	};

	stopPolling = () => {
		if (this.pollInterval) {
			clearInterval(this.pollInterval);
			this.pollInterval = null;
		}
	};
}
