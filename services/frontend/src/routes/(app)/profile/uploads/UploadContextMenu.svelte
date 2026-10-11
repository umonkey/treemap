<script lang="ts">
	import BaseContextMenu from '$lib/components/menu/BaseContextMenu.svelte';
	import type { IContextMenuItem } from '$lib/components/menu/BaseContextMenu.svelte.ts';
	import TrashIcon from '$lib/icons/TrashIcon.svelte';

	const {
		visible,
		onDelete,
		onClose
	}: {
		visible: boolean;
		onDelete: () => void;
		onClose: () => void;
	} = $props();

	const items = $derived<IContextMenuItem[]>([
		{
			icon: TrashIcon,
			label: 'Delete',
			onClick: onDelete
		}
	]);
</script>

<BaseContextMenu open={visible} {items} {onClose} />
