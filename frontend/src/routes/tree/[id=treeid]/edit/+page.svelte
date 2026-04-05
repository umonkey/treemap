<script lang="ts">
	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import CanopyInput from '$lib/ui/canopy-input/CanopyInput.svelte';
	import CircumferenceInput from '$lib/ui/circumference-input/CircumferenceInput.svelte';
	import HeightInput from '$lib/ui/height-input/HeightInput.svelte';
	import LocationInput from '$lib/ui/location-input/LocationInput.svelte';
	import NotesInput from '$lib/ui/notes-input/NotesInput.svelte';
	import SpeciesInput from '$lib/ui/species-input/SpeciesInput.svelte';
	import StateInput from '$lib/ui/state-input/StateInput.svelte';
	import StreetInput from '$lib/ui/street-input/StreetInput.svelte';
	import YearInput from '$lib/ui/year-input/YearInput.svelte';
	import { hooks } from './hooks';

	const { data } = $props();

	const {
		loading,
		saving,
		tree,
		updated,
		reload,
		handleSpeciesChange,
		handleHeightChange,
		handleCircumferenceChange,
		handleStateChange,
		handleDiameterChange,
		handleNotesChange,
		handleLocationChange,
		handleConfirm,
		handleCancel,
		handleYearChange,
		handleAddressChange
	} = hooks();

	$effect(() => reload(data.id));
</script>

{#if $loading}
	<!-- Loading... -->
{:else if $tree}
	<TreeForm
		title="Edit Tree"
		id={data.id}
		onSubmit={handleConfirm}
		onCancel={handleCancel}
		disabled={$saving}
	>
		<SpeciesInput value={$updated.species} onChange={handleSpeciesChange} />
		<StateInput value={$updated.state} onChange={handleStateChange} />

		<HeightInput value={$updated.height} onChange={handleHeightChange} />
		<CanopyInput value={$updated.diameter} onChange={handleDiameterChange} />
		<CircumferenceInput value={$updated.circumference} onChange={handleCircumferenceChange} />
		<YearInput value={$updated.year} onChange={handleYearChange} />
		<LocationInput value={{ lat: $tree.lat, lng: $tree.lon }} onChange={handleLocationChange} />

		<StreetInput value={$tree.address} onChange={handleAddressChange} />

		<NotesInput value={$updated.notes} onChange={handleNotesChange} />
	</TreeForm>
{/if}
