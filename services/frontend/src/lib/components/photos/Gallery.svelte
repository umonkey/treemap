<script lang="ts">
	import FALLBACK from '$lib/assets/tree.jpg';
	import { Gallery } from './Gallery.svelte.ts';

	const { id }: { id: string } = $props();

	const gallery = new Gallery();

	$effect(() => {
		gallery.reload(id);
	});
</script>

<div class="gallery">
	{#if gallery.loading && gallery.slides.length === 0}
		<!-- Loading files -->
	{:else if gallery.error}
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
					<img src={item.small} alt="Tree" />
				</a>
			{:else}
				<div class="tile"><img src={FALLBACK} alt="No photos of this tree." /></div>
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
	}
</style>
