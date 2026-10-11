<script lang="ts">
	import BaseContextMenu from '$lib/components/menu/BaseContextMenu.svelte';
	import type { IContextMenuItem } from '$lib/components/menu/BaseContextMenu.svelte.ts';
	import CheckListIcon from '$lib/icons/CheckListIcon.svelte';
	import EditIcon from '$lib/icons/EditIcon.svelte';
	import MapIcon from '$lib/icons/MapIcon.svelte';
	import SkullIcon from '$lib/icons/SkullIcon.svelte';
	import TrashIcon from '$lib/icons/TrashIcon.svelte';
	import TreeIcon from '$lib/icons/TreeIcon.svelte';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import { onMount } from 'svelte';
	import { menuState } from './TreeContextMenu.svelte.ts';

	onMount(menuState.onMount);

	const items = $derived.by((): IContextMenuItem[] => {
		const id = menuState.id;

		if (!id) {
			return [];
		}

		return [
			{
				icon: MapIcon,
				label: locale.contextMove(),
				href: routes.treeMove(id)
			},
			{
				icon: SkullIcon,
				label: locale.contextDead(),
				href: routes.treeDead(id),
				separator: true
			},
			{
				icon: TrashIcon,
				label: locale.contextGone(),
				href: routes.treeDelete(id)
			},
			{
				icon: EditIcon,
				label: locale.contextEditTree(),
				href: routes.treeEdit(id),
				separator: true
			},
			{
				icon: TreeIcon,
				label: locale.contextReplace(),
				href: routes.treeReplace(id)
			},
			{
				icon: CheckListIcon,
				label: locale.treeTabsObservations(),
				href: routes.treeObservations(id),
				separator: true
			}
		];
	});
</script>

<BaseContextMenu
	open={menuState.id !== undefined}
	position={menuState.position}
	{items}
	onClose={menuState.handleClose}
/>
