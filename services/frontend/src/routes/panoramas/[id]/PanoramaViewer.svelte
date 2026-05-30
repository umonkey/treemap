<script lang="ts">
	import type { MapillaryImage } from '$lib/api/mapillary';

	interface Props {
		image: MapillaryImage;
	}

	let { image }: Props = $props();
</script>

<div class="viewer">
	{#if image.url}
		<img src={image.url} alt="Panorama {image.id}" />
	{:else}
		<p>Loading image...</p>
	{/if}
</div>

<style>
	.viewer {
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		background-color: #000;
		overflow: hidden;

		img {
			max-width: 100%;
			max-height: 100%;
			object-fit: contain;
		}
	}
</style>
