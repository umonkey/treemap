<script lang="ts">
	import { LocationPicker, FormElement } from '$lib/ui';
	import { MapIcon } from '$lib/icons';
	import { locale } from '$lib/locale';
	import type { ILatLng } from '$lib/types';

	const { value, hint, label, onChange, open } = $props<{
		value: ILatLng;
		hint?: string;
		label?: string;
		onChange: (ll: ILatLng) => void;
		open?: boolean;
	}>();

	let showMap = $state<boolean>(!!open);
	let currentValue = $state<ILatLng>(value);

	const formatLocation = (ll: ILatLng): string => {
		return `${ll.lat.toFixed(7)}, ${ll.lng.toFixed(7)}`;
	};

	const toggleMap = () => {
		showMap = !showMap;
	};

	const round = (value: number): number => {
		return Math.round(value * 10000000) / 10000000;
	};

	const handleMove = (ll: ILatLng) => {
		if (ll.lat != currentValue.lat || ll.lng != currentValue.lng) {
			currentValue = ll;

			onChange({
				lat: round(ll.lat),
				lng: round(ll.lng)
			});
		}
	};

	$effect(() => (currentValue = value));
</script>

<FormElement label={label ?? locale.locationLabel()} {hint}>
	<div class="group">
		<input type="text" value={formatLocation(currentValue)} readonly={true} />
		<button type="button" onclick={toggleMap}><MapIcon /></button>
	</div>

	{#if showMap}
		<LocationPicker center={value} pin={value} onMove={handleMove} />
	{/if}
</FormElement>

<style>
	.group {
		display: flex;
		flex-direction: row;
		gap: var(--gap);
	}

	button {
		background: transparent;
		border: none;
		color: var(--icon-color-secondary);
		width: 30px;
		display: block;
		cursor: pointer;
	}
</style>
