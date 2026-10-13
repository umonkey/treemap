<script lang="ts">
	import CloseIcon from '$lib/icons/CloseIcon.svelte';
	import type { IGalleryItem } from '$lib/types';
	import { ImageViewerLogic } from './ImageViewer.svelte.ts';

	const { item, onClose }: { item: IGalleryItem; onClose: () => void } = $props();
	const viewer = new ImageViewerLogic();

	let imageEl: HTMLImageElement;

	$effect(() => viewer.bind(onClose));

	$effect(() => viewer.bindImage(imageEl));

	// Reset zoom/pan when the displayed image changes (the component is not remounted on prop change).
	$effect(() => {
		const id = item.id;
		if (!id) return;
		viewer.reset();
	});

	$effect(() => {
		window.addEventListener('keydown', viewer.handleKeydown);
		return () => window.removeEventListener('keydown', viewer.handleKeydown);
	});

	$effect(() => {
		const previous = document.body.style.overflow;
		document.body.style.overflow = 'hidden';
		return () => {
			document.body.style.overflow = previous;
		};
	});
</script>

<svelte:window onresize={viewer.handleResize} />

<div class="viewer" role="dialog" aria-modal="true">
	<button class="backdrop" aria-label="Close" onclick={onClose}></button>
	<img
		class="image"
		class:pannable={viewer.canPan}
		class:dragging={viewer.dragging}
		src={item.large}
		alt={item.label || 'Tree'}
		draggable="false"
		style="transform: translate({viewer.offsetX}px, {viewer.offsetY}px) scale({viewer.scale})"
		bind:this={imageEl}
		onwheel={viewer.handleWheel}
		ondblclick={viewer.handleDblClick}
		onpointerdown={viewer.handlePointerDown}
		onpointermove={viewer.handlePointerMove}
		onpointerup={viewer.handlePointerUp}
		onpointercancel={viewer.handlePointerUp}
	/>
	<button class="close" aria-label="Close" onclick={onClose}><CloseIcon /></button>
</div>

<style>
	.viewer {
		position: fixed;
		inset: 0;
		z-index: var(--z-viewer);
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.9);
		animation: fadeIn 0.2s ease-out;
		overflow: hidden;
		touch-action: none;
	}

	.backdrop {
		position: absolute;
		inset: 0;
		border: none;
		margin: 0;
		padding: 0;
		background: transparent;
		cursor: pointer;
	}

	.image {
		position: relative;
		z-index: 1;
		max-width: 100%;
		max-height: 100%;
		object-fit: contain;
		transform-origin: center center;
		will-change: transform;
		user-select: none;
		-webkit-user-drag: none;
		touch-action: none;
		cursor: default;
	}

	.image.pannable {
		cursor: grab;
	}

	.image.pannable.dragging {
		cursor: grabbing;
	}

	.close {
		position: absolute;
		z-index: 2;
		top: 1rem;
		right: 1rem;
		width: 32px;
		height: 32px;
		padding: 4px;
		display: flex;
		align-items: center;
		justify-content: center;
		border: none;
		border-radius: 5px;
		background: rgba(0, 0, 0, 0.4);
		color: #fff;
		cursor: pointer;
	}
</style>
