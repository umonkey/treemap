<script lang="ts">
	import CloseIcon from '$lib/icons/CloseIcon.svelte';
	import type { IGalleryItem } from '$lib/types';
	import { ImageViewerLogic } from './ImageViewer.svelte.ts';

	const { item, onClose }: { item: IGalleryItem; onClose: () => void } = $props();
	const viewer = new ImageViewerLogic();

	$effect(() => viewer.bind(onClose));

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

<div class="viewer" role="dialog" aria-modal="true">
	<button class="backdrop" aria-label="Close" onclick={onClose}></button>
	<img class="image" src={item.large} alt={item.label || 'Tree'} />
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
