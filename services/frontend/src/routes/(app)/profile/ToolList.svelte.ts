import { get } from 'svelte/store';
import { hasPermission } from '$lib/stores/authStore';

export class ToolListLogic {
	items = $derived.by(() => {
		const check = get(hasPermission);
		const list: { name: string; href: string }[] = [];

		if (check('user:manage') || check('pano:edit')) {
			list.push({
				name: 'Admin',
				href: '/admin'
			});
		}

		if (check('tree:create')) {
			list.push({
				name: 'Manual trilateration',
				href: '/tools/range'
			});
		}

		return list;
	});
}
