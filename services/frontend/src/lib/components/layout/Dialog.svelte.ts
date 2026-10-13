import { goto, routes } from '$lib/routes';
import { sidebarState } from '$lib/stores/sidebar.svelte';

export class DialogState {
	query = $state('');

	/** Widen the map's left gutter for as long as the dialog is open. */
	init = () => {
		sidebarState.wide = true;
		return () => {
			sidebarState.wide = false;
		};
	};

	handleInput = (value: string) => {
		this.query = value;
	};

	handleSearch = async (value: string) => {
		await goto(routes.searchQuery(value));
	};

	handleClose = async () => {
		await goto(routes.home());
	};
}
