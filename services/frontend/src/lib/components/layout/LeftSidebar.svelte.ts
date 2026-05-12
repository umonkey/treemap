import { mobileSidebarStore } from '$lib/stores/mobileSidebarStore';
import { pwaStore } from '$lib/stores/pwaStore';
import { get } from 'svelte/store';

class ComponentState {
	public close = () => {
		mobileSidebarStore.set(false);
	};

	public install = () => {
		const event = get(pwaStore);
		if (event) {
			event.prompt();
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
