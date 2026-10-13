<script lang="ts">
	import FALLBACK from '$lib/assets/tree.jpg';
	import SpinnerIcon from '$lib/icons/SpinnerIcon.svelte';
	import { routes } from '$lib/routes';
	import CameraUploader from './CameraUploader.svelte';
	import { Gallery } from './Gallery.svelte.ts';
	import GalleryUploader from './GalleryUploader.svelte';

	const { id, count }: { id: string; count?: number | undefined } = $props();

	const gallery = new Gallery();

	// The files are fetched here, but the caller usually already knows how many
	// photos to expect: reserve that many tiles while loading so the grid does
	// not jump once the images arrive.
	const placeholders = $derived(gallery.loading ? (count ?? 0) : 0);

	$effect(() => {
		gallery.reload(id);
	});

	$effect(() => {
		return gallery.watchQueue(id);
	});

	$effect(() => {
		return gallery.watchUploads(id);
	});

	const handleLoad = (e: Event) => {
		const target = e.currentTarget as HTMLImageElement;
		target.style.opacity = '1';
	};
</script>

<div class="gallery">
	{#if gallery.error}
		<p>{gallery.error}</p>
	{/if}

	<div class="grid">
		<CameraUploader treeId={id} />
		<GalleryUploader treeId={id} />

		{#each gallery.queue as item (item.id)}
			<a class="tile queued" class:busy={item.busy} href={routes.uploads()} title="Uploading">
				<img src={item.src} alt="Uploading" onload={handleLoad} />
				{#if item.busy}
					<div class="spinner"><SpinnerIcon /></div>
				{/if}
			</a>
		{/each}

		{#if !gallery.error}
			{#each gallery.slides as item (item.id)}
				<a
					class="tile"
					href={item.large}
					target="_blank"
					rel="noopener noreferrer"
					title={item.label}
				>
					<img src={item.small} loading="lazy" alt="Tree" onload={handleLoad} />
				</a>
			{:else}
				{#if placeholders > 0}
					{#each Array(placeholders).keys() as index (index)}
						<div class="tile placeholder"></div>
					{/each}
				{:else if !gallery.loading && gallery.queue.length === 0}
					<div class="tile"><img src={FALLBACK} alt="No photos of this tree." /></div>
				{/if}
			{/each}
		{/if}
	</div>
</div>

<style>
	.gallery {
		width: 100%;
	}

	.grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 8px;
	}

	.tile {
		position: relative;
		display: block;
		aspect-ratio: 1/1;
		overflow: hidden;

		&.queued {
			opacity: 0.5;
		}
	}

	.tile.queued .spinner {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 25%;
		background-color: rgba(0, 0, 0, 0.5);
	}

	.tile img {
		display: block;
		width: 100%;
		height: 100%;
		object-fit: cover;
		color: transparent;
		background-color: rgba(0, 0, 0, 0.2);
		opacity: 0;
		transition: opacity 0.1s ease-in-out;
	}

	.tile.placeholder {
		background-color: rgba(0, 0, 0, 0.2);
	}
</style>
