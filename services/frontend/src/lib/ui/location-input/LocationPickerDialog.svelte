<script lang="ts">
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import { MapLibre, Marker } from 'svelte-maplibre';
	import 'maplibre-gl/dist/maplibre-gl.css';
	import TreeLayer from '$lib/components/map/TreeLayer.svelte';
	import { LocationPickerState } from './LocationPickerDialog.svelte.ts';
	import type { ILatLng } from '$lib/types';

	const {
		value,
		onSelect,
		onCancel
	}: {
		value?: ILatLng | null;
		onSelect: (val: ILatLng) => void;
		onCancel: () => void;
	} = $props();

	const state = new LocationPickerState();

	$effect(() => {
		if (value) {
			state.selectedLocation = { ...value };
		}
	});

	const confirm = () => {
		if (state.selectedLocation) {
			onSelect(state.selectedLocation);
		}
	};

	const buttons = [
		{
			title: 'Use My Location',
			onClick: state.useMyLocation
		},
		{
			title: 'Confirm',
			onClick: confirm
		}
	];
</script>

<Dialog title="Select Location" {onCancel} {buttons}>
	<div class="picker-container">
		<div class="map-wrapper">
			<MapLibre
				style={state.layer}
				bind:map={state.map}
				class="map"
				center={value ? [value.lng, value.lat] : [44.5152, 40.1872]}
				zoom={15}
				onclick={state.handleMapClick}
				onload={() => state.fitInitial(value)}
				attributionControl={false}
			>
				<TreeLayer />
				{#if state.selectedLocation}
					<Marker lngLat={[state.selectedLocation.lng, state.selectedLocation.lat]}>
						<div class="picker-marker"></div>
					</Marker>
				{/if}
			</MapLibre>
		</div>
		<div class="hint">Click on the map to set location</div>
	</div>
</Dialog>

<style>
	.picker-container {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		width: 100%;
		height: 100%;
	}

	.map-wrapper {
		width: 100%;
		height: 400px;
		border-radius: 8px;
		overflow: hidden;
		position: relative;
	}

	:global(.map) {
		width: 100%;
		height: 100%;
	}

	.picker-marker {
		width: 20px;
		height: 20px;
		background-color: #ff4500;
		border: 3px solid #fff;
		border-radius: 50%;
		box-shadow: 0 0 6px rgba(0, 0, 0, 0.5);
	}

	.hint {
		font-size: 0.9rem;
		color: var(--pico-muted-color);
		text-align: center;
	}
</style>
