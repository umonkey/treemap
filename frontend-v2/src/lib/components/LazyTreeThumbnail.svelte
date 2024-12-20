<script lang="ts">
	import { routes } from '$lib/routes';

	const { tree } = $props();

	const fallback = '/tree.jpg';
	const src = tree.thumbnail_id ? routes.file(tree.thumbnail_id) : fallback;
	const alt = tree.species;

	let img;

	const handleError = () => {
		if (img.src !== fallback) {
			console.debug(`Error loading image ${src}, falling back to ${fallback}`);
			img.src = fallback;
		}
	};

	const handleLoad = () => {
		img.style.opacity = '1';
	};
</script>

<img {src} {alt} loading="lazy" onerror={() => handleError()} onload={handleLoad} bind:this={img} />

<style>
	img {
		width: 100%;
		aspect-ratio: 1/1;
		object-fit: cover;
		object-position: center;
		opacity: 0;
		transition: opacity 0.1s ease-in-out;
	}
</style>
