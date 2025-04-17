<script lang="ts">
const { src, alt, fallback } = $props();

let img: HTMLImageElement;

const handleError = () => {
	console.debug(`Error loading image ${src}, falling back to ${fallback}`);
	img.src = fallback;
};

const handleLoad = () => {
	img.style.opacity = "1";
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
