<script lang="ts">
	/**
	 * This form lets the user fill in details for the new tree,
	 * sends the tree to the API, then reports success to the container.
	 */

	import { hooks } from './hooks';
	import { locale } from '$lib/locale';
	import type { ILatLng } from '$lib/types';
	import { Form, Map, MapFullscreen, MapRow } from '$lib/ui';

	import {
		Button,
		Buttons,
		CanopyInput,
		CircumferenceInput,
		HeightInput,
		LocationInput,
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
		loading,
		location,
		tree,
		handleSpeciesChange,
		handleHeightChange,
		handleDiameterChange,
		handleCircumferenceChange,
		handleStateChange,
		handleNotesChange,
		handleYearChange,
		handleConfirm,
		handleCancel,
		saving
	} = hooks();
</script>

{#if $loading}
	<!-- loading ... -->
{:else}
	<Form>
		<Map center={start} zoom={19}>
			<MapFullscreen />
		</Map>

		<LocationInput
			value={$location}
			label={locale.addConfirmLocation()}
			onChange={handleLocationChange}
		/>
		<SpeciesInput value={$tree.species} onChange={handleSpeciesChange} />
		<HeightInput value={$tree.height} onChange={handleHeightChange} />
		<CanopyInput value={$tree.diameter} onChange={handleDiameterChange} />
		<CircumferenceInput value={$tree.circumference} onChange={handleCircumferenceChange} />
		<StateInput value={$tree.state} onChange={handleStateChange} />
		<YearInput value={$tree.year} onChange={handleYearChange} />
		<NotesInput value={$tree.notes} onChange={handleNotesChange} />

		<Buttons>
			<Button type="submit" onClick={handleConfirm} disabled={$saving}
				>{locale.addConfirmButton()}</Button
			>
			<Button type="cancel" onClick={handleCancel} disabled={$saving}
				>{locale.addCancelButton()}</Button
			>
		</Buttons>
	</Form>
{/if}
