<script lang="ts">
	/**
	 * This form lets the user fill in details for the new tree,
	 * sends the tree to the API, then reports success to the container.
	 */

	import { hooks } from './hooks';
	import { locale } from '$lib/locale';
	import type { ILatLng } from '$lib/types';
	import { Form, MapRowPreview, RowSizeInput } from '$lib/ui';

	import {
		Button,
		Buttons,
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

<Form>
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

	<Buttons>
		<Button type="submit" onClick={handleConfirm} disabled={$saving}
			>{locale.addRowConfirmButton()}</Button
		>
		<Button type="cancel" onClick={handleCancel} disabled={$saving}
			>{locale.addCancelButton()}</Button
		>
	</Buttons>
</Form>

<style>
	p {
		margin: 0;
	}
</style>
