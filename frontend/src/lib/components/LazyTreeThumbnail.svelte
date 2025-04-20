<script lang="ts">
	import { routes } from '$lib/routes';
	import type { ITree } from '$lib/types';
	import FALLBACK from '$lib/assets/tree.jpg';

	const { tree } = $props<{
		tree: ITree;
	}>();

	const src = tree.thumbnail_id ? routes.file(tree.thumbnail_id) : FALLBACK;
	const alt = tree.species;

	let img: HTMLImageElement;

	const handleError = () => {
		if (img.src !== FALLBACK) {
			console.debug(`Error loading image ${src}, falling back to ${FALLBACK}`);
			img.src = FALLBACK;
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
