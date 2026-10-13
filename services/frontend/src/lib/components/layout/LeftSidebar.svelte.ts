import { goto, routes } from '$lib/routes';
import { mobileSidebarStore } from '$lib/stores/mobileSidebarStore';
import { pwaStore } from '$lib/stores/pwaStore';
import { get } from 'svelte/store';

class ComponentState {
	query = $state('');

	public close = () => {
		mobileSidebarStore.set(false);
	};

	public handleInput = (value: string) => {
		this.query = value;
	};

	public handleSearch = async (value: string) => {
		await goto(routes.searchQuery(value));
	};

	public handleClose = async () => {
		await goto(routes.home());
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
