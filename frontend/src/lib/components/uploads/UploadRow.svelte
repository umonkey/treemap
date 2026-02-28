<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
	import type { IUpload } from '$lib/db';

	const { upload } = $props<{
		upload: IUpload;
	}>();

	let objectUrl = $state<string>('');

	onMount(() => {
		objectUrl = URL.createObjectURL(upload.image);
	});

	onDestroy(() => {
		if (objectUrl) {
			URL.revokeObjectURL(objectUrl);
		}
	});

	const formatSize = (bytes: number) => {
		return (bytes / 1024 / 1024).toFixed(2) + ' MB';
	};
</script>

<div class="upload-item">
	{#if objectUrl}
		<img src={objectUrl} alt="Thumbnail" class="thumbnail" />
	{/if}
	<div class="overlay">
		{formatSize(upload.image.size)} - {upload.status}
	</div>
</div>

<style>
	.upload-item {
		position: relative;
		aspect-ratio: 1 / 1;
		width: 100%;
		overflow: hidden;
		border-radius: 4px;
		border: 1px solid rgba(128, 128, 128, 0.5);
	}

	.thumbnail {
		width: 100%;
		height: 100%;
		object-fit: cover;
		display: block;
	}

	.overlay {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		background: rgba(0, 0, 0, 0.5);
		color: white;
		padding: 4px 8px;
		font-size: 0.8rem;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
		text-transform: capitalize;
	}
</style>
