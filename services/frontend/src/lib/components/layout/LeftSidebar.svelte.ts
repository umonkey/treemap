import { getStats } from '$lib/api/stats';
import { locale } from '$lib/locale';
import { mobileSidebarStore } from '$lib/stores/mobileSidebarStore';
import { pwaStore } from '$lib/stores/pwaStore';
import type { IStats } from '$lib/types';
import { toast } from 'svelte-sonner';
import { get } from 'svelte/store';

class ComponentState {
	public stats = $state<IStats | null>(null);

	public async fetchStats() {
		const res = await getStats();
		if (res.data) {
			this.stats = res.data;
		}
	}

	public close = () => {
		mobileSidebarStore.set(false);
	};

	public install = () => {
		const event = get(pwaStore);
		if (event) {
			event.prompt();
			this.close();
		} else {
			toast.info(locale.sideInstallManualInstructions(), {
				duration: 10000
			});
			this.close();
		}
	};

	public handleOverlayClick = (e: MouseEvent) => {
		if (e.target === e.currentTarget) {
			this.close();
		}
	};

	public static componentState = new ComponentState();
}

export const componentState = ComponentState.componentState;
