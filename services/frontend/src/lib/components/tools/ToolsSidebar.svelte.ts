import { get } from 'svelte/store';
import { hasPermission } from '$lib/stores/authStore';
import { goto, routes } from '$lib/routes';
import { mobileSidebarStore } from '$lib/stores/mobileSidebarStore';

export class ToolsSidebarLogic {
	query = $state('');

	items = $derived.by(() => {
		const check = get(hasPermission);
		const list: { name: string; href: string }[] = [];

		if (check('user:manage')) {
			list.push({
				name: 'Users',
				href: '/admin/users'
			});
		}

		if (check('pano:edit')) {
			list.push({
				name: 'Panoramas',
				href: '/admin/panoramas'
			});
		}

		if (check('tree:create')) {
			list.push({
				name: 'Trilaterate',
				href: '/tools/range'
			});
		}

		if (check('tree:delete')) {
			list.push({
				name: 'Duplicates',
				href: '/tools/merge'
			});
		}

		list.push({
			name: 'Learn',
			href: '/learn'
		});

		return list;
	});

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
