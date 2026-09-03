<script lang="ts">
	import { locale } from '$lib/locale';
	import type { ILatLng } from '$lib/types';
	import FormElement from '$lib/ui/form-element/FormElement.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import LocationPickerDialog from './LocationPickerDialog.svelte';

	const {
		value,
		hint,
		label,
		onChange,
		onClear
	}: {
		value?: ILatLng | null;
		hint?: string;
		label?: string;
		onChange?: (val: ILatLng) => void;
		onClear?: () => void;
	} = $props();

	let showPicker = $state(false);

	const formatLocation = (ll?: ILatLng | null): string => {
		if (!ll || Number.isNaN(ll.lat) || Number.isNaN(ll.lng)) return '';
		return `${ll.lat.toFixed(7)}, ${ll.lng.toFixed(7)}`;
	};

	const formattedLocation = $derived(formatLocation(value));
</script>

<FormElement label={label ?? locale.locationLabel()} {hint}>
	<div class="group">
		<input type="text" value={formattedLocation} readonly={true} placeholder="Not set" />
		<Button type="secondary" onClick={() => (showPicker = true)}>Select on map</Button>
		{#if value && onClear}
			<Button type="danger" onClick={onClear}>Clear</Button>
		{/if}
	</div>
</FormElement>

{#if showPicker}
	<LocationPickerDialog
		{value}
		onSelect={(val) => {
			onChange?.(val);
			showPicker = false;
		}}
		onCancel={() => (showPicker = false)}
	/>
{/if}

<style>
	.group {
		display: flex;
		flex-direction: row;
		gap: var(--gap);
		align-items: center;
	}
</style>
