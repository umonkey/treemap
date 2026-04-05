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
	import { pageState } from './page.svelte';

	const { data } = $props();

	$effect(() => {
		pageState.reload(data.id);
	});
</script>

{#if pageState.loading}
	<!-- Loading... -->
{:else if pageState.tree}
	<TreeForm
		title="Edit Tree"
		id={data.id}
		onSubmit={pageState.handleConfirm}
		onCancel={pageState.handleCancel}
		saving={pageState.saving}
	>
		<SpeciesInput value={pageState.updated.species} onChange={pageState.handleSpeciesChange} />
		<StateInput value={pageState.updated.state} onChange={pageState.handleStateChange} />

		<HeightInput value={pageState.updated.height} onChange={pageState.handleHeightChange} />
		<CanopyInput value={pageState.updated.diameter} onChange={pageState.handleDiameterChange} />
		<CircumferenceInput
			value={pageState.updated.circumference}
			onChange={pageState.handleCircumferenceChange}
		/>
		<YearInput value={pageState.updated.year} onChange={pageState.handleYearChange} />
		<LocationInput
			value={{ lat: pageState.tree.lat, lng: pageState.tree.lon }}
			onChange={pageState.handleLocationChange}
		/>

		<StreetInput value={pageState.tree.address} onChange={pageState.handleAddressChange} />

		<NotesInput value={pageState.updated.notes} onChange={pageState.handleNotesChange} />
	</TreeForm>
{/if}
