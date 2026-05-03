import { mobileSidebarStore } from '$lib/stores/mobileSidebarStore';

class BarsButtonLogic {
	/**
	 * Opens the mobile sidebar.
	 */
	openSidebar = () => {
		mobileSidebarStore.set(true);
	};
}

export const componentState = new BarsButtonLogic();
