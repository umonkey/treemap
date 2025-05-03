<script lang="ts">
	/**
	 * This form lets the user fill in details for the new tree,
	 * sends the tree to the API, then reports success to the container.
	 */

	import { hooks } from './hooks';
	import { locale } from '$lib/locale';

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

	const { lat, lng } = $props<{
		lat: number | null;
		lng: number | null;
	}>();

	const {
		loading,
		location,
		tree,
		reload,
		handleSpeciesChange,
		handleHeightChange,
		handleDiameterChange,
		handleCircumferenceChange,
		handleStateChange,
		handleNotesChange,
		handleLocationChange,
		handleYearChange,
		handleConfirm,
		handleCancel,
		saving
	} = hooks();

	$effect(() => reload(lat, lng));
</script>

{#if $loading}
	<!-- loading ... -->
{:else}
	<div class="form">
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
			<Button
				type="submit"
				label={locale.addConfirmButton()}
				onClick={handleConfirm}
				disabled={$saving}
			/>
			<Button
				type="cancel"
				label={locale.addCancelButton()}
				onClick={handleCancel}
				disabled={$saving}
			/>
		</Buttons>
	</div>
{/if}
