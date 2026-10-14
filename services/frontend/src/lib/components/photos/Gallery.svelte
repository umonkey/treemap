<script lang="ts">
	import SpinnerIcon from '$lib/icons/SpinnerIcon.svelte';
	import UploadIcon from '$lib/icons/UploadIcon.svelte';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import CameraUploader from './CameraUploader.svelte';
	import { Gallery } from './Gallery.svelte.ts';
	import GalleryUploader from './GalleryUploader.svelte';
	import ImageViewer from './ImageViewer.svelte';

	const {
		id,
		count,
		species
	}: { id: string; count?: number | undefined; species?: string | undefined } = $props();

	const gallery = new Gallery();

	// The files are fetched here, but the caller usually already knows how many
	// photos to expect: reserve that many tiles while loading so the grid does
	// not jump once the images arrive.
	const placeholders = $derived(gallery.loading ? (count ?? 0) : 0);

	// No photos to show once loading settled, nothing is queued and no error is up.
	const isEmpty = $derived(
		!gallery.error && !gallery.loading && gallery.slides.length === 0 && gallery.queue.length === 0
	);

	$effect(() => {
		gallery.reload(id, species ?? '');
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

<div
	class="gallery"
	role="group"
	ondragover={gallery.handleDragOver}
	ondragenter={gallery.handleDragEnter}
	ondragleave={gallery.handleDragLeave}
	ondrop={(event) => gallery.handleDrop(id, event)}
>
	{#if gallery.error}
		<p>{gallery.error}</p>
	{/if}

	{#if isEmpty}
		<p class="empty">{locale.noPhotos()}</p>
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
				<button
					type="button"
					class="tile"
					onclick={() => gallery.openViewer(item)}
					title={item.label}
				>
					<img src={item.small} loading="lazy" alt="Tree" onload={handleLoad} />
					{#if item.label}
						<span class="badge">{item.label}</span>
					{/if}
				</button>
			{:else}
				{#each Array(placeholders).keys() as index (index)}
					<div class="tile placeholder"></div>
				{/each}
			{/each}
		{/if}
	</div>

	{#if gallery.selected}
		<ImageViewer item={gallery.selected} onClose={gallery.closeViewer} />
	{/if}

	{#if gallery.dragging}
		<div class="drop-zone" aria-hidden="true">
			<span class="drop-icon"><UploadIcon /></span>
			<span>{locale.dropPhotos()}</span>
		</div>
	{/if}
</div>

<style>
	.gallery {
		position: relative;
		width: 100%;
	}

	.drop-zone {
		position: absolute;
		inset: 0;
		z-index: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 8px;
		box-sizing: border-box;
		padding: 16px;
		text-align: center;
		border: 2px dashed var(--form-border);
		border-radius: 8px;
		background-color: rgba(128, 128, 128, 0.9);
		color: #fff;
		font-weight: 600;
		pointer-events: none;

		.drop-icon {
			display: block;
			width: 32px;
			height: 32px;

			:global(svg) {
				width: 100%;
				height: 100%;
			}
		}
	}

	.empty {
		margin: 0 0 8px;
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
		background: none;
		border: none;
		padding: 0;
		font: inherit;
		cursor: pointer;

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

	.tile .badge {
		position: absolute;
		bottom: 4px;
		left: 4px;
		padding: 2px 6px;
		border-radius: 4px;
		background-color: rgba(0, 0, 0, 0.55);
		color: #fff;
		font-size: 0.8rem;
		line-height: 1.2;
	}
</style>
