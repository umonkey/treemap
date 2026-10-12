<script lang="ts">
	import FALLBACK from '$lib/assets/tree.jpg';
	import { Gallery } from './Gallery.svelte.ts';

	const { id, count }: { id: string; count?: number | undefined } = $props();

	const gallery = new Gallery();

	// The files are fetched here, but the caller usually already knows how many
	// photos to expect: reserve that many tiles while loading so the grid does
	// not jump once the images arrive.
	const placeholders = $derived(gallery.loading ? (count ?? 0) : 0);

	$effect(() => {
		gallery.reload(id);
	});

	const handleLoad = (e) => {
		e.target.style.opacity = '1';
	};
</script>

<div class="gallery">
	{#if gallery.error}
		<p>{gallery.error}</p>
	{:else}
		<div class="grid">
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
				{:else if !gallery.loading}
					<div class="tile"><img src={FALLBACK} alt="No photos of this tree." /></div>
				{/if}
			{/each}
		</div>
	{/if}
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
		display: block;
		aspect-ratio: 1/1;
		overflow: hidden;
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
