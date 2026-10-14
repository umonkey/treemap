import { page } from '$app/state';
import { goto, routes } from '$lib/routes';
import { mobileSidebarStore } from '$lib/stores/mobileSidebarStore';

export class ToolsSidebarLogic {
	query = $state('');

	public isActive = (href: string) => {
		const path = page.url.pathname;
		return path === href || path.startsWith(`${href}/`);
	};

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

	public handleOverlayClick = (e: MouseEvent) => {
		if (e.target === e.currentTarget) {
			this.close();
		}
	};
}
