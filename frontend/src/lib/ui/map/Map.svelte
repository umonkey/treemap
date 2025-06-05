<script lang="ts">
	import 'leaflet/dist/leaflet.css';
	import { baseLayer } from '$lib/stores/mapLayerStore';
	import { onDestroy, onMount, type Snippet } from 'svelte';
	import { hook } from './hooks';
	import CROSSHAIR from '$lib/assets/crosshair.svg';
	import type { ILatLng } from '$lib/types';

	const {
		center,
		className = 'default',
		pins,
		searchQuery = undefined,
		crosshair = false,
		canAdd = false,
		children = undefined
	} = $props<{
		center: [number, number];
		className: string;
		pins?: ILatLng[];
		searchQuery?: string | undefined;
		crosshair?: boolean | undefined;
		canAdd?: boolean | undefined;
		children?: Snippet | undefined;
	}>();

	const { handleCenter, handlePinsChange, handleSearch, handleCanAdd, handleElementChange } = hook(
		'map',
		onMount,
		onDestroy
	);

	let map: HTMLDivElement = $state<HTMLDivElement | undefined>(undefined);

	$effect(() => handleCenter(center));
	$effect(() => handlePinsChange(pins));
	$effect(() => handleCanAdd(canAdd));
	$effect(() => handleSearch(searchQuery));
	$effect(() => handleElementChange(map));
</script>

<div class="wrapper">
	<div
		id="map"
		bind:this={map}
		class={className}
		class:dark={$baseLayer === 'OSM Dark'}
		class:light={$baseLayer !== 'OSM Dark'}
	></div>

	{#if children && map}
		{@render children()}
	{/if}

	{#if crosshair}
		<img src={CROSSHAIR} class="crosshair" alt="" />
	{/if}
</div>

<style>
	.wrapper {
		width: 100%;
		height: 100%;
		position: relative;
	}

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
		z-index: var(--z-crosshair);
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
