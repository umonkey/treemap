<script lang="ts">
	import CameraIcon from '$lib/icons/CameraIcon.svelte';
	import GalleryIcon from '$lib/icons/GalleryIcon.svelte';
	import FileUploaderDisplay from '$lib/ui/file-uploader-display/FileUploaderDisplay.svelte';
	import { restartUploadQueue } from '$lib/upload';
	import { onMount } from 'svelte';
	import { load } from './PhotoUploader';

	const { label, treeId, onChange, small } = $props<{
		label?: string;
		treeId: string;
		onChange: (files: number) => void;
		small?: boolean;
	}>();

	const { thumbnails, handleChange } = load({ treeId, onChange, onMount });
</script>

{#if label}
	<div class="label">{label}</div>
{/if}

<div class="uploader" class:small={!!small}>
	<label>
		<CameraIcon />

		<input type="file" accept="image/jpeg" onchange={handleChange} capture="environment" multiple />
	</label>

	<label class="gallery">
		<GalleryIcon />

		<input type="file" accept="image/jpeg" onchange={handleChange} multiple />
	</label>

	<FileUploaderDisplay
		items={$thumbnails.map((thumbnail) => ({
			src: URL.createObjectURL(thumbnail.file),
			busy: thumbnail.busy,
			error: thumbnail.error
		}))}
		onRetry={restartUploadQueue}
	/>

	<div class="filler"></div>
</div>

<style>
	.label {
		margin: 0 0 var(--gap) 0;
		opacity: 0.75;
	}

	.uploader {
		display: flex;
		flex-direction: row;
		gap: 10px;
		justify-content: normal;

		height: 100px;

		/* Testing mobile UI in Storybook */
		&.small {
			height: 50px;
		}
	}

	label {
		height: 100%;
		aspect-ratio: 1;

		margin: 0;
		padding: 0;

		background-color: #444;
		padding: 10px;
		cursor: pointer;
		border-radius: 8px;
		box-sizing: border-box;

		input {
			visibility: hidden;
			width: 0;
			height: 0;
		}
	}

	.filler {
		flex-shrink: 1;
		flex-grow: 1;
	}

	@media (max-width: 1023px) {
		.uploader {
			height: 50px;
		}
	}

	@media (min-width: 1024px) {
		.gallery {
			display: none;
		}
	}
</style>
