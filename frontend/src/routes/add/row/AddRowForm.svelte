<script lang="ts">
	/**
	 * This form lets the user fill in details for the new tree,
	 * sends the tree to the API, then reports success to the container.
	 */

	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import { locale } from '$lib/locale';
	import type { ILatLng } from '$lib/types';
	import RowSizeInput from '$lib/ui/row-size-input/RowSizeInput.svelte';
	import MapRowPreview from './MapRowPreview.svelte';
	import { hooks } from './hooks';

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

	const {
		count,
		step,
		distance,
		handleConfirm,
		handleCancel,
		handleCountChange,
		saving,
		handleSpeciesChange,
		handleHeightChange,
		handleDiameterChange,
		handleCircumferenceChange,
		handleStateChange,
		handleYearChange,
		handleNotesChange
	} = hooks({
		start,
		end
	});
</script>

<TreeForm
	title="Add Row of Trees"
	onSubmit={handleConfirm}
	onCancel={handleCancel}
	saving={$saving}
>
	<MapRowPreview {start} {end} count={$count} />

	<RowSizeInput value={$count} {distance} onChange={handleCountChange} />

	<SpeciesInput value="" onChange={handleSpeciesChange} />
	<StateInput value="unknown" onChange={handleStateChange} />
	<HeightInput value={0} onChange={handleHeightChange} />
	<CanopyInput value={0} onChange={handleDiameterChange} />
	<CircumferenceInput value={0} onChange={handleCircumferenceChange} />
	<YearInput value={null} onChange={handleYearChange} />
	<NotesInput value={null} onChange={handleNotesChange} />

	<p>{locale.rowStepInfo($count, $step)}</p>
</TreeForm>

<style>
	p {
		margin: 0;
	}
</style>
