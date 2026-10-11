<script lang="ts">
	import BaseContextMenu from '$lib/components/menu/BaseContextMenu.svelte';
	import type { IContextMenuItem } from '$lib/components/menu/BaseContextMenu.svelte.ts';
	import CreateIcon from '$lib/icons/CreateIcon.svelte';
	import TrashIcon from '$lib/icons/TrashIcon.svelte';

	const {
		visible,
		position = undefined,
		onMakeThumbnail,
		onDelete,
		onClose
	}: {
		visible: boolean;
		position?: { x: number; y: number };
		onMakeThumbnail: () => void;
		onDelete: () => void;
		onClose: () => void;
	} = $props();

	const items = $derived<IContextMenuItem[]>([
		{
			icon: CreateIcon,
			label: 'Make thumbnail',
			onClick: onMakeThumbnail
		},
		{
			icon: TrashIcon,
			label: 'Delete',
			onClick: onDelete
		}
	]);
</script>

<BaseContextMenu open={visible} {items} {onClose} {position} />
