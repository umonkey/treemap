<script lang="ts">
	import CameraIcon from '$lib/icons/CameraIcon.svelte';
	import GalleryIcon from '$lib/icons/GalleryIcon.svelte';
	import { isTouchDevice } from '$lib/utils/device';
	import { locale } from './lang';
	import { PhotoPickerLogic } from './PhotoPicker.svelte.ts';

	const { onFiles, disabled = false }: { onFiles: (files: File[]) => void; disabled?: boolean } =
		$props();

	const componentState = new PhotoPickerLogic();
</script>

<div class="picker">
	<label class="tile" class:disabled={!!disabled} title={locale.takePhoto()}>
		<CameraIcon />

		<input
			type="file"
			accept="image/jpeg,image/png,image/heic,image/heif,image/webp"
			capture="environment"
			multiple
			disabled={!!disabled}
			aria-label={locale.takePhoto()}
			onchange={(e) => componentState.handleChange(e, onFiles)}
		/>
	</label>

	{#if isTouchDevice()}
		<label class="tile gallery" class:disabled={!!disabled} title={locale.pickFromGallery()}>
			<GalleryIcon />

			<input
				type="file"
				accept="image/jpeg,image/png,image/heic,image/heif,image/webp"
				multiple
				disabled={!!disabled}
				aria-label={locale.pickFromGallery()}
				onchange={(e) => componentState.handleChange(e, onFiles)}
			/>
		</label>
	{/if}
</div>

<style>
	.picker {
		display: flex;
		flex-direction: row;
		gap: 10px;
	}

	.tile {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 34px;
		margin: 0;
		padding: 6px 12px;
		box-sizing: border-box;
		border-radius: 6px;
		background-color: var(--map-primary-background);
		color: #fff;
		cursor: pointer;

		&.disabled {
			cursor: default;
			opacity: 0.5;
		}

		:global(svg) {
			display: block;
			width: 18px;
			height: 18px;
		}
	}

	input {
		visibility: hidden;
		width: 0;
		height: 0;
	}

	@media (min-width: 1024px) {
		.gallery {
			display: none;
		}
	}
</style>
