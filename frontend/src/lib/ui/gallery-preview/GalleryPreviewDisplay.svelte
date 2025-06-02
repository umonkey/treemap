<script lang="ts">
	import DefaultImage from '$lib/assets/tree.jpg';
	import { LazyImage } from '$lib/ui';
	import { UploadIcon } from '$lib/icons';
	import { routes } from '$lib/routes';

	const { loading, error, mapper, tree_id, images } = $props<{
		loading: boolean;
		error?: string;
		mapper: boolean;
		tree_id: string;
		images: string[];
	}>();
</script>

<div class="gallery" class:loading class:error={!!error} class:mapper>
	{#if !!error}
		<p>{error}</p>
	{:else if !!loading}
		<!-- loading -->
	{:else}
		<div class="images">
			{#if mapper}
				<div class="tile">
					<a href={routes.treeUploadPhotos(tree_id)} class="upload" title="Upload a new image">
						<UploadIcon />
					</a>
				</div>
			{/if}

			{#each images as image}
				<div class="tile">
					<a href={routes.treeDetails(tree_id)} aria-labelledby="thumbnail">
						<LazyImage src={image} alt="See how good is this tree." fallback={DefaultImage} />
					</a>
				</div>
			{/each}

			{#if images.length === 0 && !mapper}
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
		}
	}
</style>
