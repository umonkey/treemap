<script lang="ts">
	import CameraIcon from '$lib/icons/CameraIcon.svelte';
	import GalleryIcon from '$lib/icons/GalleryIcon.svelte';
	import FileUploaderDisplay from '$lib/ui/file-uploader-display/FileUploaderDisplay.svelte';
	import { load } from './hooks';

	const { label, onBusy, onChange, small, single } = $props<{
		label?: string;
		onChange: (files: string[]) => void;
		onBusy: (busy: boolean) => void;
		small?: boolean;
		single?: boolean;
	}>();

	const { items, handleChange, handleRetry } = load({
		onBusy,
		onChange
	});
</script>

{#if label}
	<div class="label">{label}</div>
{/if}

<div class="uploader" class:small={!!small}>
	<label title="Take Photo">
		<CameraIcon />

		<input
			type="file"
			accept="image/jpeg,image/png,image/heic,image/heif,image/webp"
			onchange={handleChange}
			capture="environment"
			multiple={!single}
		/>
	</label>

	<label class="gallery" title="Pick from Gallery">
		<GalleryIcon />

		<input
			type="file"
			accept="image/jpeg,image/png,image/heic,image/heif,image/webp"
			onchange={handleChange}
			multiple={!single}
		/>
	</label>

	<FileUploaderDisplay
		items={$items.map((item) => ({
			src: URL.createObjectURL(item.file),
			busy: item.uploading,
			error: item.error
		}))}
		onRetry={handleRetry}
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
