import { getPanorama, exportPanorama, type Panorama } from '$lib/api/panoramas';
import type { IError } from '$lib/types';

export class PageState {
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

	public onMount = () => {
		window.addEventListener('focus', this.handleFocus);
		document.addEventListener('visibilitychange', this.handleVisibilityChange);
		return () => {
			window.removeEventListener('focus', this.handleFocus);
			document.removeEventListener('visibilitychange', this.handleVisibilityChange);
		};
	};

	private handleFocus = () => {
		if (document.visibilityState === 'hidden') return;
		if (this.currentId && !this.isLoading) {
			this.reload(this.currentId, { silent: true });
		}
	};

	private handleVisibilityChange = () => {
		this.handleFocus();
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

	exportData = async (id: string) => {
		this.stopPolling();
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
