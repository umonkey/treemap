<script lang="ts">
	import GallerySlides from '$lib/ui/gallery-slides/GallerySlides.svelte';
	import { hooks } from './Gallery';

	const { id, initialImageId } = $props<{
		id: string;
		initialImageId?: string;
	}>();
	const { loading, error, slides, reload } = hooks();

	$effect(() => reload(id));
</script>

<div class="gallery">
	{#if $loading}
		<!-- Loading files -->
	{:else if $error}
		<p>{$error}</p>
	{:else}
		<GallerySlides slides={$slides} {initialImageId} />
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
