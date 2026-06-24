<script lang="ts">
	import { pageState } from './page.svelte.ts';
	import { page } from '$app/state';
	import { formatDateTime } from '$lib/utils/strings';
	import { mapLayerStore } from '$lib/stores/mapLayerStore';
	import CloseIcon from '$lib/icons/CloseIcon.svelte';
	import PlusIcon from '$lib/icons/PlusIcon.svelte';
	import TrashIcon from '$lib/icons/TrashIcon.svelte';
	import FullScreenIcon from '$lib/icons/FullScreenIcon.svelte';
	import CrossHair from '$lib/icons/CrossHair.svelte';
	import PanoramaViewer from './PanoramaViewer.svelte';

	const id = $derived(page.params.id as string);
	let previewElement = $state<HTMLElement>();

	const capturedAt = $derived(
		pageState.image?.captured_at ? formatDateTime(pageState.image.captured_at) : ''
	);

	$effect(() => {
		pageState.reload(id);
	});

	$effect(() => {
		return pageState.cleanup;
	});

	function toggleFullscreen() {
		if (!document.fullscreenElement) {
			previewElement?.requestFullscreen();
		} else {
			document.exitFullscreen();
		}
	}
</script>

<svelte:head>
	<title>360 Panorama</title>
</svelte:head>

<div class="preview" bind:this={previewElement}>
	<div class="header">
		<div class="top-left">
			<button
				type="button"
				class="control fullscreen"
				onclick={toggleFullscreen}
				aria-label="Fullscreen"
			>
				<FullScreenIcon />
			</button>
		</div>
		<div class="top-right">
			<button
				type="button"
				class="control close"
				onclick={pageState.handleClose}
				aria-label="Close"
			>
				<CloseIcon />
			</button>
		</div>
	</div>

	{#if $mapLayerStore.treeHints}
		<div class="middle-right">
			<button
				type="button"
				class="control add"
				onclick={pageState.handleAddTree}
				disabled={pageState.isBusy}
				aria-label="Add Tree"
			>
				<PlusIcon />
			</button>
			<button
				type="button"
				class="control delete"
				onclick={pageState.handleDeleteTrees}
				disabled={pageState.isBusy}
				aria-label="Delete Trees"
			>
				<TrashIcon />
			</button>
		</div>
	{/if}

	<div class="content">
		{#if pageState.image}
			<PanoramaViewer
				image={pageState.image}
				trees={$mapLayerStore.treeHints ? pageState.trees : []}
				angle={pageState.angle}
				onMove={pageState.handleMove}
			/>
			<div class="crosshair">
				<CrossHair />
			</div>
			{#if capturedAt}
				<div class="control timestamp">
					{capturedAt}
				</div>
			{/if}
		{/if}
	</div>
</div>

<style>
	.preview {
		z-index: 2;
		display: flex;
		flex-direction: column;
		gap: var(--gap);
		padding: 0;
		background-color: var(--map-menu-background);
		box-sizing: border-box;
		position: relative;

		&:fullscreen {
			width: 100vw;
			height: 100vh;
			border-radius: 0;
			border: none;
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

	.timestamp {
		position: absolute;
		bottom: 0;
		left: 0;
		width: auto;
		height: auto;
		padding: 4px 8px;
		font-size: 12px;
		z-index: 1;
		cursor: default;
		background-color: rgba(0, 0, 0, 0.75);
		color: white;
		border-radius: 0;

		&:hover {
			background-color: #000;
		}
	}

	.content {
		flex-grow: 1;
		min-height: 0;
		display: flex;
		flex-direction: column;
		position: relative;
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

	/* Mobile styles */
	@media screen and (max-width: 1023px) {
		.preview {
			position: fixed;
			bottom: var(--bottom-nav-height);
			left: 0;
			right: 0;
			height: 300px;
			border-top-left-radius: 8px;
			border-top-right-radius: 8px;
			animation: slideUp 0.2s ease-out;
		}
	}

	/* Desktop styles */
	@media screen and (min-width: 1024px) {
		.preview {
			position: fixed;
			bottom: var(--gap);
			left: var(--gap);
			width: 400px;
			height: 300px;
			border-right: 1px solid var(--color-dialog-border);
		}
	}

	@keyframes slideUp {
		from {
			transform: translateY(100%);
		}
		to {
			transform: translateY(0);
		}
	}
</style>
