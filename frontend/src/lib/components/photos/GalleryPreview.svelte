<script lang="ts">
	// This is the gallery preview shown on the map when you click a tree.
	// It shows the first 3 images of the tree, if any, or a default image.
	//
	// In the mapper mode, clicking on the default image redirects to the
	// adding photo page.
	//
	// By default, trees returned in batches don't contain files, so we have to
	// request tree details again.

	import DefaultImage from '$lib/assets/tree.jpg';
	import UploadIcon from '$lib/icons/UploadIcon.svelte';
	import { routes } from '$lib/routes';
	import type { ITreeFile } from '$lib/types';
	import LazyImage from '$lib/ui/lazy-image/LazyImage.svelte';
	import { galleryPreviewState } from './GalleryPreview.svelte.ts';

	const {
		id,
		loading,
		error,
		mapper = true,
		files
	} = $props<{
		id: string;
		loading?: boolean;
		error?: string;
		mapper?: boolean;
		files?: ITreeFile[];
	}>();

	$effect(() => {
		galleryPreviewState.reload(id);
	});

	const actualLoading = $derived(loading !== undefined ? loading : galleryPreviewState.loading);
	const actualError = $derived(error !== undefined ? error : galleryPreviewState.error);
	const actualFiles = $derived(files !== undefined ? files : galleryPreviewState.files);
</script>

<div class="gallery" class:loading={actualLoading} class:error={!!actualError} class:mapper>
	{#if !!actualError}
		<p>{actualError}</p>
	{:else if actualLoading}
		<div class="images">
			<div class="tile placeholder"></div>
			<div class="tile placeholder"></div>
			<div class="tile placeholder"></div>
			<div class="tile placeholder"></div>
			<div class="tile placeholder"></div>
		</div>
	{:else}
		<div class="images">
			{#if mapper}
				<div class="tile">
					<a href={routes.treeUploadPhotos(id)} class="upload" title="Upload a new image">
						<UploadIcon />
					</a>
				</div>
			{/if}

			{#each actualFiles as file}
				<div class="tile">
					<a href={routes.treeDetails(id, file.id)} aria-labelledby="thumbnail">
						<LazyImage
							src={routes.file(file.small_id)}
							alt="See how good is this tree."
							fallback={DefaultImage}
						/>
					</a>
				</div>
			{/each}

			{#if actualFiles.length === 0 && !mapper}
				<div class="tile">
					<a href={routes.treeDetails(id)} title="No photos of this tree.">
						<LazyImage src={DefaultImage} alt="No photos for this tree." fallback={DefaultImage} />
					</a>
				</div>
			{/if}
		</div>
	{/if}
</div>

<style>
	.gallery {
		height: 75px;
		line-height: 75px;
	}

	.images {
		display: flex;
		flex-direction: row;
		gap: var(--gap);

		overflow-x: auto;
		scroll-snap-type: x mandatory;
		scrollbar-width: none;

		.tile {
			width: 75px;
			height: 75px;
			border-radius: 4px;
			overflow: hidden;
			flex-shrink: 0;
			flex-grow: 0;
			background-color: var(--color-dialog-header);
			color: var(--color-dialog-text);

			a {
				display: block;
				width: 75px;
				height: 75px;

				color: inherit;
				line-height: 0;

				scroll-snap-align: start;
				scroll-snap-stop: always;

				&.upload {
					padding: 20px;
					box-sizing: border-box;
				}
			}

			&.placeholder {
				background-color: #000;
				animation: pulse 1.5s infinite ease-in-out;
			}
		}
	}

	@keyframes pulse {
		0% {
			opacity: 1;
		}
		50% {
			opacity: 0.4;
		}
		100% {
			opacity: 1;
		}
	}
</style>
