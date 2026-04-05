<script lang="ts">
	import DefaultImage from '$lib/assets/tree.jpg';
	import UploadIcon from '$lib/icons/UploadIcon.svelte';
	import { routes } from '$lib/routes';
	import type { ITreeFile } from '$lib/types';
	import LazyImage from '$lib/ui/lazy-image/LazyImage.svelte';

	const { loading, error, mapper, tree_id, files } = $props<{
		loading: boolean;
		error?: string;
		mapper: boolean;
		tree_id: string;
		files: ITreeFile[];
	}>();
</script>

<div class="gallery" class:loading class:error={!!error} class:mapper>
	{#if !!error}
		<p>{error}</p>
	{:else if loading}
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
					<a href={routes.treeUploadPhotos(tree_id)} class="upload" title="Upload a new image">
						<UploadIcon />
					</a>
				</div>
			{/if}

			{#each files as file}
				<div class="tile">
					<a href={routes.treeDetails(tree_id, file.id)} aria-labelledby="thumbnail">
						<LazyImage
							src={routes.file(file.small_id)}
							alt="See how good is this tree."
							fallback={DefaultImage}
						/>
					</a>
				</div>
			{/each}

			{#if files.length === 0 && !mapper}
				<div class="tile">
					<a href={routes.treeDetails(tree_id)} title="No photos of this tree.">
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
			background-color: light-dark(var(--pico-color-grey-500), var(--pico-color-grey-950));
			color: white;

			a {
				display: block;
				width: 75px;
				height: 75px;

				color: inherit;
				line-height: 0;

				scroll-snap-align: start;
				scroll-snap-stop: always;

				&.upload {
					background-color: rgba(0, 0, 0, 0.25);
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
