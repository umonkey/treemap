<script lang="ts">
	import type { PanoramaImage, PanoramaHint } from '$lib/api/panoramas';
	import { componentState } from './PanoramaViewer.svelte.ts';
	import { untrack } from 'svelte';
	import 'pannellum/build/pannellum.css';
	import CloseIcon from '$lib/icons/CloseIcon.svelte';
	import PlusIcon from '$lib/icons/PlusIcon.svelte';
	import TrashIcon from '$lib/icons/TrashIcon.svelte';
	import FullScreenIcon from '$lib/icons/FullScreenIcon.svelte';
	import CrossHair from '$lib/icons/CrossHair.svelte';

	interface Props {
		image: PanoramaImage;
		angle?: number;
		trees?: PanoramaHint[];
		onMove?: (angle: number) => void;
		onTreeClick?: (treeId: string) => void;
		onImageClick?: (imageId: string) => void;
		onAddHint?: () => void;
		onDeleteHints?: () => void;
		onClose?: () => void;
		isBusy?: boolean;
	}

	const {
		image,
		angle = 0,
		trees = [],
		onMove,
		onTreeClick,
		onImageClick,
		onAddHint,
		onDeleteHints,
		onClose,
		isBusy = false
	}: Props = $props();

	let container = $state<HTMLElement | null>(null);
	let fullscreenElement = $state<HTMLElement | null>(null);

	$effect(() => {
		if (container && image.url) {
			const initialYaw = untrack(() => angle);
			componentState.init(
				container,
				image,
				initialYaw,
				onMove,
				onTreeClick,
				onImageClick,
				onAddHint
			);
		}
		return () => {
			componentState.destroy();
		};
	});

	$effect(() => {
		componentState.setTrees(trees);
	});

	$effect(() => {
		window.addEventListener('keydown', componentState.handleKeydown);
		return () => {
			window.removeEventListener('keydown', componentState.handleKeydown);
		};
	});

	$effect(() => {
		console.log('New angle:', componentState.yaw);
	});
</script>

<div class="panorama-viewer" bind:this={fullscreenElement}>
	<div class="viewer" bind:this={container}>
		{#if !image.url}
			<p>Loading image...</p>
		{/if}
	</div>

	<div class="header">
		<div class="top-left">
			<button
				type="button"
				class="control fullscreen"
				onclick={() => componentState.toggleFullscreen(fullscreenElement)}
				aria-label="Fullscreen"
			>
				<FullScreenIcon />
			</button>
		</div>
		{#if onClose}
			<div class="top-right">
				<button type="button" class="control close" onclick={onClose} aria-label="Close">
					<CloseIcon />
				</button>
			</div>
		{/if}
	</div>

	{#if onAddHint || onDeleteHints}
		<div class="middle-right">
			{#if onAddHint}
				<button
					type="button"
					class="control add"
					onclick={onAddHint}
					disabled={isBusy}
					aria-label="Add Tree"
				>
					<PlusIcon />
				</button>
			{/if}
			{#if onDeleteHints}
				<button
					type="button"
					class="control delete"
					onclick={onDeleteHints}
					disabled={isBusy}
					aria-label="Delete Trees"
				>
					<TrashIcon />
				</button>
			{/if}
		</div>
	{/if}

	<div class="crosshair">
		<CrossHair />
	</div>
</div>

<style>
	.panorama-viewer {
		position: relative;
		width: 100%;
		height: 100%;

		&:fullscreen {
			width: 100vw;
			height: 100vh;
			background-color: #000;
		}
	}

	.header {
		position: absolute;
		top: 10px;
		left: 10px;
		right: 10px;
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		background-color: transparent;
		z-index: 1;
	}

	.middle-right {
		position: absolute;
		top: 50%;
		right: 10px;
		transform: translateY(-50%);
		display: flex;
		flex-direction: column;
		background-color: white;
		color: black;
		border-radius: 4px;
		overflow: hidden;
		box-shadow: 0 0 0 2px rgba(0, 0, 0, 0.1);
		z-index: 1;

		.control + .control {
			border-top: 1px solid #ddd;
		}
	}

	.top-left,
	.top-right {
		display: flex;
		flex-direction: column;
		background-color: white;
		color: black;
		border-radius: 4px;
		overflow: hidden;
		box-shadow: 0 0 0 2px rgba(0, 0, 0, 0.1);
	}

	.control {
		width: 29px;
		height: 29px;
		cursor: pointer;
		background-color: transparent;
		border: none;
		color: inherit;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 0;

		&:hover {
			background-color: rgba(0, 0, 0, 0.05);
		}

		&:disabled {
			opacity: 0.5;
			cursor: not-allowed;
		}

		:global(svg) {
			width: 20px;
			height: 20px;
		}
	}

	.crosshair {
		position: absolute;
		left: 50%;
		top: 50%;
		z-index: 10;
		transform: translate(-50%, -50%);
		width: 50px;
		height: 50px;
		pointer-events: none;
		color: white;
		filter: drop-shadow(0 0 2px rgba(0, 0, 0, 0.5));

		:global(svg) {
			width: 100%;
			height: 100%;
			fill: currentColor;
		}

		:global(.cls-1) {
			fill: currentColor;
		}
	}

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
