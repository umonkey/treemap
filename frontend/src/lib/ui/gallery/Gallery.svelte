<script lang="ts">
	import { hooks } from './hooks';
	import { GallerySlides } from '$lib/ui';

	const { id } = $props<{ id: string }>();
	const { loading, error, slides, reload } = hooks();

	$effect(() => reload(id));
</script>

<div class="gallery">
	{#if $loading}
		<!-- Loading files -->
	{:else if $error}
		<p>{$error}</p>
	{:else}
		<GallerySlides slides={$slides} />
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
