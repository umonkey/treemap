<script lang="ts">
	/**
	 * This form lets the user fill in details for the new tree,
	 * sends the tree to the API, then reports success to the container.
	 */

	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import { locale } from '$lib/locale';
	import { hooks } from './hooks';

	import CanopyInput from '$lib/ui/canopy-input/CanopyInput.svelte';
	import CircumferenceInput from '$lib/ui/circumference-input/CircumferenceInput.svelte';
	import HeightInput from '$lib/ui/height-input/HeightInput.svelte';
	import LocationInput from '$lib/ui/location-input/LocationInput.svelte';
	import NotesInput from '$lib/ui/notes-input/NotesInput.svelte';
	import SpeciesInput from '$lib/ui/species-input/SpeciesInput.svelte';
	import StateInput from '$lib/ui/state-input/StateInput.svelte';
	import YearInput from '$lib/ui/year-input/YearInput.svelte';

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
	<TreeForm title="Add Tree" onSubmit={handleConfirm} onCancel={handleCancel} saving={$saving}>
		<LocationInput
			value={$location}
			label={locale.addConfirmLocation()}
			onChange={handleLocationChange}
		/>
		<SpeciesInput value={$tree.species} onChange={handleSpeciesChange} />
		<StateInput value={$tree.state} onChange={handleStateChange} />
		<HeightInput value={$tree.height} onChange={handleHeightChange} />
		<CanopyInput value={$tree.diameter} onChange={handleDiameterChange} />
		<CircumferenceInput value={$tree.circumference} onChange={handleCircumferenceChange} />
		<YearInput value={$tree.year} onChange={handleYearChange} />
		<NotesInput value={$tree.notes} onChange={handleNotesChange} />
	</TreeForm>
{/if}
