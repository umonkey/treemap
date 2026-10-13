<script lang="ts">
	import CameraIcon from '$lib/icons/CameraIcon.svelte';
	import { hasPermission } from '$lib/stores/authStore';
	import { CameraUploader } from './CameraUploader.svelte.ts';

	const { treeId }: { treeId: string } = $props();

	const state = new CameraUploader();
</script>

{#if $hasPermission('tree:edit')}
	<label class="tile camera" title="Take Photo">
		<CameraIcon />

		<input
			type="file"
			accept="image/jpeg,image/png,image/heic,image/heif,image/webp"
			capture="environment"
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
