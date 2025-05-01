<script lang="ts">
	import 'leaflet/dist/leaflet.css';
	import { baseLayer } from '$lib/stores/mapLayerStore';
	import { onDestroy, onMount } from 'svelte';
	import { hook } from './hooks';
	import CROSSHAIR from '$lib/assets/crosshair.svg';

	const {
		center,
		className = 'default',
		marker,
		searchQuery = undefined,
		crosshair = false,
		canAdd = false
	} = $props<{
		center: [number, number];
		className: string;
		marker?: [number, number];
		searchQuery?: string | undefined;
		crosshair?: boolean | undefined;
		canAdd?: boolean | undefined;
	}>();

	const { handleCenter, handleMarkers, handleCanAdd } = hook('map', onMount, onDestroy);

	$effect(() => handleCenter(center));
	$effect(() => handleMarkers(marker));
	$effect(() => handleCanAdd(canAdd));
</script>

<div
	id="map"
	class={className}
	class:dark={$baseLayer === 'OSM Dark'}
	class:light={$baseLayer !== 'OSM Dark'}
></div>

{#if crosshair}
	<img src={CROSSHAIR} class="crosshair" alt="" />
{/if}

<style>
	#map {
		height: 100%;
		width: 100%;
		z-index: 1;
	}

	#map.light {
		background-color: #f9f6ef;
	}

	#map.dark {
		background-color: #080808;
	}

	.crosshair {
		width: 30px;
		height: 30px;
		position: absolute;
		left: 50%;
		top: 50%;
		z-index: 50;
		transform: translate(-50%, -50%);
	}

	:global {
		.cluster-count {
			background-color: transparent;
			border: none;
			box-shadow: none;
			font-size: 14px;
			line-height: 40px;
			text-align: center;
			padding: 0;
			color: #040;
		}
	}

	@media (max-width: 480px) {
		:global {
			.cluster-count {
				font-size: 14px;
			}
		}
	}

	@media (prefers-color-scheme: dark) {
		:global {
			.cluster-count {
				color: #000;
			}

			#map.dark .cluster-count {
				color: #fff;
			}
		}
	}
</style>
