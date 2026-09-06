<script lang="ts">
	import type { PanoramaImage, PanoramaHint } from '$lib/api/panoramas';
	import { componentState } from './PanoramaViewer.svelte.ts';
	import { untrack } from 'svelte';
	import 'pannellum/build/pannellum.css';

	interface Props {
		image: PanoramaImage;
		angle?: number;
		trees?: PanoramaHint[];
		onMove?: (angle: number) => void;
		onTreeClick?: (treeId: string) => void;
		onImageClick?: (imageId: string) => void;
	}

	const { image, angle = 0, trees = [], onMove, onTreeClick, onImageClick }: Props = $props();

	let container = $state<HTMLElement | null>(null);

	$effect(() => {
		if (container && image.url) {
			const initialYaw = untrack(() => angle);
			componentState.init(container, image, initialYaw, onMove, onTreeClick, onImageClick);
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

	:global(.pnlm-hotspot-base.tree-marker-disc) {
		width: 30px;
		height: 30px;
		border-radius: 50%;
		background-color: #22c55e !important;
		border: 2px solid #ffffff !important;
		background-image: none !important;
		box-sizing: border-box;
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;
	}

	:global(.tree-marker-disc svg) {
		width: 16px;
		height: 16px;
		color: #000000;
	}

	:global(.pnlm-hotspot-base.image-marker-disc) {
		width: 30px;
		height: 30px;
		border-radius: 50%;
		background-color: #60a5fa !important;
		border: 2px solid #ffffff !important;
		background-image: none !important;
		box-sizing: border-box;
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;
	}

	:global(.image-marker-disc svg) {
		width: 16px;
		height: 16px;
		color: #000000;
	}
</style>
