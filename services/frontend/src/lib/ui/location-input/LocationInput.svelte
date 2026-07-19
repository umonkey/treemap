<script lang="ts">
	import { locale } from '$lib/locale';
	import type { ILatLng } from '$lib/types';
	import FormElement from '$lib/ui/form-element/FormElement.svelte';

	const {
		value,
		hint,
		label
	}: {
		value: ILatLng;
		hint?: string;
		label?: string;
	} = $props();

	const formatLocation = (ll: ILatLng): string => {
		return `${ll.lat.toFixed(7)}, ${ll.lng.toFixed(7)}`;
	};

	const formattedLocation = $derived(formatLocation(value));
</script>

<FormElement label={label ?? locale.locationLabel()} {hint}>
	<div class="group">
		<input type="text" value={formattedLocation} readonly={true} />
	</div>
</FormElement>

<style>
	.group {
		display: flex;
		flex-direction: row;
		gap: var(--gap);
	}
</style>
