<script lang="ts">
	import { pageState } from './page.svelte.ts';
	import { page } from '$app/state';
	import CloseIcon from '$lib/icons/CloseIcon.svelte';
	import EditIcon from '$lib/icons/EditIcon.svelte';
	import CrossHair from '$lib/icons/CrossHair.svelte';
	import PanoramaViewer from './PanoramaViewer.svelte';

	const id = $derived(page.params.id as string);
	const capturedAt = $derived.by(() => {
		if (!pageState.image?.captured_at) return '';
		const date = new Date(pageState.image.captured_at);
		const Y = date.getFullYear();
		const M = (date.getMonth() + 1).toString().padStart(2, '0');
		const D = date.getDate().toString().padStart(2, '0');
		const h = date.getHours().toString();
		const i = date.getMinutes().toString().padStart(2, '0');
		const s = date.getSeconds().toString().padStart(2, '0');
		return `${Y}-${M}-${D}, ${h}:${i}:${s}`;
	});

	$effect(() => {
		pageState.reload(id);
	});

	$effect(() => {
		return pageState.cleanup;
	});
</script>

<svelte:head>
	<title>360 Panorama</title>
</svelte:head>

<div class="preview">
	<div class="header">
		<button type="button" class="control edit" onclick={pageState.handleDetect} aria-label="Edit">
			<EditIcon />
		</button>
		<button type="button" class="control close" onclick={pageState.handleClose} aria-label="Close">
			<CloseIcon />
		</button>
	</div>

	<div class="content">
		{#if pageState.image}
			<PanoramaViewer
				image={pageState.image}
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
	}

	.header {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		display: flex;
		align-items: center;
		justify-content: space-between;
		background-color: transparent;
		z-index: 1;

		.edit {
			border-bottom-right-radius: 25%;
		}

		.close {
			border-bottom-left-radius: 25%;
		}
	}

	.control {
		width: 30px;
		height: 30px;
		cursor: pointer;
		background-color: rgba(0, 0, 0, 0.75);
		border: none;
		color: white;
		display: flex;
		align-items: center;
		justify-content: center;

		&:hover {
			background-color: rgba(0, 0, 0, 1);
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
