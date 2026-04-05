<script lang="ts">
	import GallerySlides from '$lib/ui/gallery-slides/GallerySlides.svelte';
	import { galleryState } from './Gallery.svelte.ts';

	const { id, initialImageId } = $props<{
		id: string;
		initialImageId?: string;
	}>();

	$effect(() => {
		galleryState.reload(id);
	});
</script>

<div class="gallery">
	{#if galleryState.loading && galleryState.slides.length === 0}
		<!-- Loading files -->
	{:else if galleryState.error}
		<p>{galleryState.error}</p>
	{:else}
		<GallerySlides slides={galleryState.slides} {initialImageId} />
	{/if}
</div>

<style>
	.gallery {
		position: relative;
		box-sizing: border-box;
		width: 100%;
		aspect-ratio: 1/1;
	}
</style>
