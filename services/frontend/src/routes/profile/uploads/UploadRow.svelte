<script lang="ts">
	import { onMount } from 'svelte';
	import type { IUpload } from '$lib/db';
	import { routes } from '$lib/routes';
	import DotsIcon from '$lib/icons/DotsIcon.svelte';

	const {
		upload,
		onMenu
	}: {
		upload: IUpload;
		onMenu: (upload: IUpload) => void;
	} = $props();

	let objectUrl = $state<string>('');

	onMount(() => {
		objectUrl = URL.createObjectURL(upload.thumbnail || upload.image);

		return () => {
			URL.revokeObjectURL(objectUrl);
		};
	});

	const formatSize = (bytes: number) => {
		return `${(bytes / 1024 / 1024).toFixed(2)} MB`;
	};
</script>

<div class="upload-item">
	<a href={routes.treeUploadPhotos(upload.tree_id)} class="link">
		{#if objectUrl}
			<img src={objectUrl} alt="Thumbnail" class="thumbnail" />
		{/if}
		{#if upload.status === 'uploading' && upload.progress !== undefined}
			<div class="progress-container">
				<div class="progress-bar" style="width: {upload.progress}%"></div>
			</div>
		{/if}
		<div class="overlay">
			{formatSize(upload.image.size)} - {upload.status}
		</div>
	</a>

	<button class="menu-trigger" onclick={() => onMenu(upload)} type="button">
		<DotsIcon />
	</button>
</div>

<style>
	.progress-container {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		height: 6px;
		background: var(--color-progress-bg, rgba(0, 0, 0, 0.3));
		z-index: 1;
	}

	.progress-bar {
		height: 100%;
		background: var(--color-progress-fg, #95d237);
		transition: width 0.3s ease;
	}

	.upload-item {
		position: relative;
		aspect-ratio: 1 / 1;
		width: 100%;
		overflow: hidden;
		border-radius: 4px;
		border: 1px solid rgba(128, 128, 128, 0.5);
	}

	.link {
		display: block;
		width: 100%;
		height: 100%;
		text-decoration: none;
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

	.menu-trigger {
		position: absolute;
		top: 0;
		right: 0;
		width: 30px;
		height: 30px;
		background: rgba(0, 0, 0, 0.7);
		border: none;
		border-radius: 0 0 0 8px;
		color: #fff;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0;
		z-index: 2;

		:global(svg) {
			width: 16px;
			height: 16px;
		}

		&:hover {
			background: rgba(0, 0, 0, 0.6);
		}
	}
</style>
