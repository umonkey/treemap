import { get } from 'svelte/store';
import { hasPermission } from '$lib/stores/authStore';

export class ToolListLogic {
	items = $derived.by(() => {
		const check = get(hasPermission);
		const list: { name: string; href: string }[] = [];

		if (check('user:manage')) {
			list.push({
				name: 'Manage Users',
				href: '/admin/users'
			});
		}

		if (check('pano:edit')) {
			list.push({
				name: 'Manage Panoramas',
				href: '/admin/panoramas'
			});
		}

		if (check('tree:create')) {
			list.push({
				name: 'Manual trilateration',
				href: '/tools/range'
			});
		}

		list.push({
			name: 'Merge duplicate trees',
			href: '/tools/merge'
		});

		list.push({
			name: 'Training',
			href: '/learn'
		});

		return list;
	});
}
