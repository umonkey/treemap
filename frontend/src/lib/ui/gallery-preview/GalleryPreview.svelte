<script lang="ts">
	// This is the gallery preview shown on the map when you click a tree.
	// It shows the first 3 images of the tree, if any, or a default image.
	//
	// In the mapper mode, clicking on the default image redirects to the
	// adding photo page.
	//
	// By default, trees returned in batches don't contain files, so we have to
	// request tree details again.

	import type { ITree } from '$lib/types';
	import { UploadIcon } from '$lib/icons';
	import { isMapperMode } from '$lib/stores/modeStore';
	import { loadTree } from '$lib/hooks';
	import { routes } from '$lib/routes';
	import LazyImage from '$lib/components/LazyImage.svelte';
	import DefaultImage from '$lib/assets/tree.jpg';

	const { tree } = $props<{
		tree: ITree;
	}>();

	const { loading, data, error, reload } = loadTree();

	$effect(() => {
		if (tree) {
			(async () => await reload(tree.id))();
		}
	});
</script>

<div class="gallery" class:loading={$loading} class:error={$error} class:mapper={$isMapperMode}>
	{#if $error}
		<p>{$error.description}</p>
	{:else if $loading}
		<!-- loading -->
	{:else if $data}
		<div class="images">
			{#if $isMapperMode}
				<div>
					<a href={routes.treeUploadPhotos(tree.id)} class="upload" title="Upload a new image">
						<UploadIcon />
					</a>
				</div>
			{/if}

			{#each $data.files as file}
				<div>
					<a href={routes.treeDetails(tree.id)} aria-labelledby="thumbnail">
						<LazyImage
							src={routes.file(file.small_id)}
							alt="See how good is this tree."
							fallback={DefaultImage}
						/>
					</a>
				</div>
			{/each}

			{#if $data.files.length === 0 && !$isMapperMode}
				<div>
					<a href={routes.treeDetails(tree.id)} title="No photos of this tree.">
						<LazyImage src={DefaultImage} alt="No photos for this tree." fallback={DefaultImage} />
					</a>
				</div>
			{/if}
		</div>
	{:else}
		<p>Failed to load photos.</p>
	{/if}
</div>

<style>
	.gallery {
		height: 50px;
		line-height: 50px;
		margin-top: var(--gap);
	}

	.images {
		display: flex;
		flex-direction: row;
		gap: var(--gap);

		scroll-snap-type: x mandatory;
		scrollbar-width: none;

		& > div {
			width: 50px;
			height: 50px;
		}

		a {
			display: block;
			color: inherit;
			line-height: 0;

			scroll-snap-align: start;
			scroll-snap-stop: always;

			&.upload {
				background-color: rgba(0, 0, 0, 0.25);
				padding: 5px;
			}
		}
	}
</style>
