<script lang="ts">
	import type { MapillaryImage } from '$lib/api/mapillary';
	import { componentState } from './PanoramaViewer.svelte.ts';
	import { untrack } from 'svelte';
	import 'pannellum/build/pannellum.css';

	interface Props {
		image: MapillaryImage;
		angle?: number;
		onMove?: (angle: number) => void;
	}

	const { image, angle = 0, onMove }: Props = $props();

	let container = $state<HTMLElement | null>(null);

	$effect(() => {
		if (container && image.url) {
			const initialYaw = untrack(() => angle);
			componentState.init(container, image, initialYaw, onMove);
		}
		return () => {
			componentState.destroy();
		};
	});

	$effect(() => {
		console.log('New angle:', componentState.yaw);
	});
</script>

<div class="viewer" bind:this={container}>
	{#if !image.url}
		<p>Loading image...</p>
	{/if}
</div>

<style>
	.viewer {
		width: 100%;
		height: 100%;
		min-height: 200px;
		background-color: #000;
		overflow: hidden;
	}

	:global(.pnlm-container) {
		background-color: #000;
	}
</style>
