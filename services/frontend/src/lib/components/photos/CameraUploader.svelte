<script lang="ts">
	import CameraIcon from '$lib/icons/CameraIcon.svelte';
	import { hasPermission } from '$lib/stores/authStore';
	import { CameraUploader } from './CameraUploader.svelte.ts';
	import { locale } from './photos.lang';

	const { treeId }: { treeId: string } = $props();

	const state = new CameraUploader();
</script>

{#if $hasPermission('tree:edit')}
	<label class="tile camera" title={locale.takePhoto()} aria-label={locale.takePhoto()}>
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
		display: flex;
		align-items: center;
		justify-content: center;
		width: 50px;
		height: 50px;
		padding: 0;
		box-sizing: border-box;
		border-radius: 50%;
		overflow: hidden;
		background-color: var(--link-color);
		color: #fff;
		cursor: pointer;
		box-shadow: 0 2px 6px rgba(0, 0, 0, 0.3);
		transition: box-shadow 0.15s ease;

		&:hover {
			box-shadow: 0 4px 10px rgba(0, 0, 0, 0.35);
		}

		:global(svg) {
			display: block;
			width: 26px;
			height: 26px;
		}
	}

	input {
		visibility: hidden;
		width: 0;
		height: 0;
	}
</style>
