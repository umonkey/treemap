<script lang="ts">
	/**
	 * This form lets the user fill in details for the new tree,
	 * sends the tree to the API, then reports success to the container.
	 */

	import { hooks } from './hooks';
	import { locale } from '$lib/locale';
	import type { ILatLng } from '$lib/types';
	import { MapRowPreview, RowSizeInput } from '$lib/ui';
	import TreeForm from '$lib/components/forms/TreeForm.svelte';

	import {
		CanopyInput,
		CircumferenceInput,
		HeightInput,
		NotesInput,
		SpeciesInput,
		StateInput,
		YearInput
	} from '$lib/ui';

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
	<HeightInput value={0} onChange={handleHeightChange} />
	<CanopyInput value={0} onChange={handleDiameterChange} />
	<CircumferenceInput value={0} onChange={handleCircumferenceChange} />
	<StateInput value="unknown" onChange={handleStateChange} />
	<YearInput value={null} onChange={handleYearChange} />
	<NotesInput value={null} onChange={handleNotesChange} />

	<p>{locale.rowStepInfo($count, $step)}</p>
</TreeForm>

<style>
	p {
		margin: 0;
	}
</style>
