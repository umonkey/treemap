<script lang="ts">
	/**
	 * This form lets the user fill in details for the new tree,
	 * sends the tree to the API, then reports success to the container.
	 */

	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import { locale } from '$lib/locale';
	import type { ILatLng } from '$lib/types';
	import MapRowPreview from './MapRowPreview.svelte';
	import RowSizeInput from './RowSizeInput.svelte';
	import { pageState } from './page.svelte';

	import CanopyInput from '$lib/ui/canopy-input/CanopyInput.svelte';
	import CircumferenceInput from '$lib/ui/circumference-input/CircumferenceInput.svelte';
	import HeightInput from '$lib/ui/height-input/HeightInput.svelte';
	import NotesInput from '$lib/ui/notes-input/NotesInput.svelte';
	import SpeciesInput from '$lib/ui/species-input/SpeciesInput.svelte';
	import StateInput from '$lib/ui/state-input/StateInput.svelte';
	import YearInput from '$lib/ui/year-input/YearInput.svelte';

	const { start, end } = $props<{
		start: ILatLng;
		end: ILatLng;
	}>();

	$effect(() => {
		pageState.init(start, end);
	});
</script>

<TreeForm
	title="Add Row of Trees"
	onSubmit={pageState.handleConfirm}
	onCancel={pageState.handleCancel}
	saving={pageState.saving}
>
	<MapRowPreview {start} {end} count={pageState.count} />

	<RowSizeInput
		value={pageState.count}
		distance={pageState.distance}
		onChange={pageState.handleCountChange}
	/>

	<SpeciesInput value="" onChange={pageState.handleSpeciesChange} />
	<StateInput value="unknown" onChange={pageState.handleStateChange} />
	<HeightInput value={0} onChange={pageState.handleHeightChange} />
	<CanopyInput value={0} onChange={pageState.handleDiameterChange} />
	<CircumferenceInput value={0} onChange={pageState.handleCircumferenceChange} />
	<YearInput value={null} onChange={pageState.handleYearChange} />
	<NotesInput value={null} onChange={pageState.handleNotesChange} />

	<p>{locale.rowStepInfo(pageState.count, pageState.step)}</p>
</TreeForm>

<style>
	p {
		margin: 0;
	}
</style>
