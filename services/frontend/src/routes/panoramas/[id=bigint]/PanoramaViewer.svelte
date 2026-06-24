<script lang="ts">
	import type { MapillaryImage, MapillaryTree } from '$lib/api/mapillary';
	import { componentState } from './PanoramaViewer.svelte.ts';
	import { untrack } from 'svelte';
	import 'pannellum/build/pannellum.css';

	interface Props {
		image: MapillaryImage;
		angle?: number;
		trees?: MapillaryTree[];
		onMove?: (angle: number) => void;
	}

	const { image, angle = 0, trees = [], onMove }: Props = $props();

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
		componentState.setTrees(trees);
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

	:global(.tree-marker) {
		width: 2px;
		height: 2000px;
		background-color: rgba(34, 197, 94, 0.8) !important;
		border: none !important;
		pointer-events: none;
		background-image: none !important;
	}

	:global(.pnlm-hotspot.tree-marker) {
		cursor: default;
	}
</style>
