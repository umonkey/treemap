<script lang="ts">
	import BaseContextMenu from '$lib/components/menu/BaseContextMenu.svelte';
	import type { IContextMenuItem } from '$lib/components/menu/BaseContextMenu.svelte.ts';
	import TrashIcon from '$lib/icons/TrashIcon.svelte';
	import { UploadContextMenuLogic } from './UploadContextMenu.svelte.ts';

	const componentState = new UploadContextMenuLogic();

	$effect(() => componentState.subscribe());

	const items = $derived<IContextMenuItem[]>([
		{ icon: TrashIcon, label: 'Delete', onClick: componentState.handleDelete }
	]);
</script>

<BaseContextMenu
	open={componentState.open}
	position={componentState.position}
	{items}
	onClose={componentState.handleClose}
/>
