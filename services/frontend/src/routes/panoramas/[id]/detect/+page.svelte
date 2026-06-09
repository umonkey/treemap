<script lang="ts">
	import { pageState } from './page.svelte.ts';
	import { page } from '$app/state';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import PanoramaViewer from '../PanoramaViewer.svelte';
	import CrossHair from '$lib/icons/CrossHair.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';

	const id = $derived(page.params.id as string);

	$effect(() => {
		pageState.reload(id);
	});

	$effect(() => {
		return pageState.cleanup;
	});
</script>

<svelte:head>
	<title>Detect Trees - {id}</title>
</svelte:head>

<Dialog title="Detect Trees" onCancel={pageState.handleClose} nopadding>
	{#if pageState.image}
		<div class="detect-container">
			<div class="viewer-wrapper">
				<PanoramaViewer
					image={pageState.image}
					trees={pageState.trees}
					angle={pageState.angle}
					onMove={pageState.handleMove}
				/>
				<div class="crosshair">
					<CrossHair />
				</div>
			</div>

			<div class="actions">
				<Buttons>
					<Button type="button" onClick={pageState.handleAddTree} disabled={pageState.isBusy}>
						Add Tree
					</Button>
					<Button
						type="secondary"
						onClick={pageState.handleDeleteTrees}
						disabled={pageState.isBusy}
					>
						Delete
					</Button>
				</Buttons>
			</div>
		</div>
	{:else}
		<p aria-busy="true">Loading image...</p>
	{/if}
</Dialog>

<style>
	.detect-container {
		display: flex;
		flex-direction: column;
		margin-top: -1rem; /* Offset Dialog's nopadding vertical padding */
	}

	.viewer-wrapper {
		width: 100%;
		aspect-ratio: 3 / 2;
		position: relative;
		background-color: #000;
	}

	@media screen and (max-width: 1023px) {
		.viewer-wrapper {
			aspect-ratio: auto;
			height: calc(100vh - 160px);
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

	.actions {
		padding: var(--gap);
		display: flex;
		justify-content: center;
		background-color: var(--map-menu-background);
		border-top: 1px solid var(--sep-color);
	}
</style>
