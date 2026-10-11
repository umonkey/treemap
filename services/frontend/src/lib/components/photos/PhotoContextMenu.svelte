<script lang="ts">
	import BaseContextMenu from '$lib/components/menu/BaseContextMenu.svelte';
	import type { IContextMenuItem } from '$lib/components/menu/BaseContextMenu.svelte.ts';
	import CreateIcon from '$lib/icons/CreateIcon.svelte';
	import TrashIcon from '$lib/icons/TrashIcon.svelte';
	import { PhotoContextMenuLogic } from './PhotoContextMenu.svelte.ts';

	const componentState = new PhotoContextMenuLogic();

	$effect(() => componentState.subscribe());

	const items = $derived<IContextMenuItem[]>([
		{ icon: CreateIcon, label: 'Make thumbnail', onClick: componentState.handleMakeThumbnail },
		{ icon: TrashIcon, label: 'Delete', onClick: componentState.handleDelete }
	]);
</script>

<BaseContextMenu
	open={componentState.open}
	position={componentState.position}
	{items}
	onClose={componentState.handleClose}
/>
