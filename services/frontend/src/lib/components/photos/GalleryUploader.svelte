<script lang="ts">
	import GalleryIcon from '$lib/icons/GalleryIcon.svelte';
	import { hasPermission } from '$lib/stores/authStore';
	import { isTouchDevice } from '$lib/utils/device';
	import { GalleryUploader } from './GalleryUploader.svelte.ts';

	const { treeId }: { treeId: string } = $props();

	const state = new GalleryUploader();
</script>

{#if $hasPermission('tree:edit') && isTouchDevice()}
	<label class="tile gallery" title="Pick from Gallery">
		<GalleryIcon />

		<input
			type="file"
			accept="image/jpeg,image/png,image/heic,image/heif,image/webp"
			multiple
			onchange={(e) => state.handleChange(treeId, e)}
		/>
	</label>
{/if}

<style>
	.tile {
		position: relative;
		display: flex;
		align-items: center;
		justify-content: center;
		aspect-ratio: 1/1;
		padding: 20%;

		background-color: var(--color-dialog-header);
		color: var(--color-dialog-text);

		cursor: pointer;
		box-sizing: border-box;
		overflow: hidden;
	}

	input {
		visibility: hidden;
		width: 0;
		height: 0;
	}
</style>
