<script lang="ts">
	import { pageState } from './page.svelte.ts';
	import { page } from '$app/state';
	import CloseIcon from '$lib/icons/CloseIcon.svelte';
	import PanoramaViewer from './PanoramaViewer.svelte';
	import { mapMarkerStore } from '$lib/stores/mapMarker.svelte';
	import { LngLat } from 'maplibre-gl';
	import { mapBus } from '$lib/buses/mapBus';
	import { mapRaysStore } from '$lib/stores/mapRays.svelte';

	const id = $derived(page.params.id as string);

	$effect(() => {
		pageState.reload(id);
	});

	$effect(() => {
		if (pageState.image) {
			const ll = { lat: pageState.image.lat, lng: pageState.image.lon };
			mapMarkerStore.center = new LngLat(ll.lng, ll.lat);
			mapBus.emit('move', ll);
		}
		return () => {
			mapMarkerStore.center = undefined;
			mapRaysStore.rays = [];
		};
	});
</script>

<div class="preview">
	<div class="header">
		<button class="close" onclick={pageState.handleClose} aria-label="Close">
			<CloseIcon />
		</button>
	</div>

	<div class="content">
		{#if pageState.image}
			<PanoramaViewer image={pageState.image} onMove={pageState.handleMove} />
		{:else}
			<p>Image ID: {pageState.id}</p>
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
		right: 0;
		align-items: center;
		justify-content: space-between;
		background-color: rgba(0, 0, 0, 0.75);
		border-bottom-left-radius: 25%;
		z-index: 1;

		.close {
			width: 30px;
			height: 30px;
			cursor: pointer;
			background-color: transparent;
			border: none;
			color: light-dark(black, white);
			opacity: 0.5;
			display: flex;
			align-items: center;
			justify-content: center;

			&:hover {
				opacity: 1;
			}
		}
	}

	.content {
		flex-grow: 1;
		min-height: 0;
		display: flex;
		flex-direction: column;
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
