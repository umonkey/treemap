<script lang="ts">
	import CameraIcon from '$lib/icons/CameraIcon.svelte';
	import GalleryIcon from '$lib/icons/GalleryIcon.svelte';
	import FileUploaderDisplay from '$lib/ui/file-uploader-display/FileUploaderDisplay.svelte';
	import { restartUploadQueue } from '$lib/upload';
	import { componentState as state } from './PhotoUploader.svelte.ts';

	const {
		label,
		treeId,
		onChange,
		small
	}: {
		label?: string;
		treeId: string;
		onChange: (files: number) => void;
		small?: boolean;
	} = $props();

	$effect(() => {
		return state.init(treeId, onChange);
	});
</script>

{#if label}
	<div class="label">{label}</div>
{/if}

{#if treeId}
	<div class="uploader" class:small={!!small} class:has-items={state.thumbnails.length > 0}>
		<div class="actions">
			<label class="camera" title="Take Photo">
				<CameraIcon />

				<input
					type="file"
					accept="image/jpeg,image/png,image/heic,image/heif,image/webp"
					onchange={state.handleChange}
					capture="environment"
				/>
			</label>

			<label class="gallery" title="Pick from Gallery">
				<GalleryIcon />

				<input
					type="file"
					accept="image/jpeg,image/png,image/heic,image/heif,image/webp"
					onchange={state.handleChange}
					multiple
				/>
			</label>
		</div>

		{#if state.thumbnails.length > 0}
			<FileUploaderDisplay
				items={state.thumbnails.map((thumbnail) => ({
					src: thumbnail.src,
					busy: thumbnail.busy,
					error: thumbnail.error
				}))}
				onRetry={restartUploadQueue}
			/>
		{/if}

		<div class="filler"></div>
	</div>
{:else}
	<p>Cannot upload files, try refreshing the page.</p>
{/if}

<style>
	.label {
		margin: 0 0 1rem 0;
		opacity: 0.75;
	}

	.uploader {
		display: flex;
		flex-direction: row;
		gap: 10px;
		justify-content: normal;

		height: 100px;
		margin-bottom: 1rem;

		/* Testing mobile UI in Storybook */
		&.small {
			height: 50px;
		}
	}

	.actions {
		display: flex;
		flex-direction: row;
		gap: 10px;
	}

	label {
		height: 100%;
		aspect-ratio: 1;

		margin: 0;
		padding: 0;

		background-color: var(--color-dialog-header);
		color: var(--color-dialog-text);

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

			&:not(.has-items) {
				height: 0;
				margin-bottom: 0;
			}
		}

		.actions {
			position: fixed;
			bottom: 1rem;
			right: 1rem;
			z-index: var(--z-add-button);

			flex-direction: column-reverse;
			gap: 1rem;
			padding: 0;
			background-color: transparent;
			border-radius: 0;

			label {
				width: 50px;
				height: 50px;
				border-radius: 50%;
				display: flex;
				align-items: center;
				justify-content: center;
				box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
				overflow: hidden;
			}
		}

		.camera {
			background-color: var(--map-primary-background);
			color: white;
		}
	}

	@media (min-width: 1024px) {
		.gallery {
			display: none;
		}
	}
</style>
