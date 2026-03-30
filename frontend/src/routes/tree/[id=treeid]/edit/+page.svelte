<script lang="ts">
	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import {
		CanopyInput,
		CircumferenceInput,
		HeightInput,
		LocationInput,
		NotesInput,
		SpeciesInput,
		StateInput,
		StreetInput,
		YearInput
	} from '$lib/ui';
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
		saving={$saving}
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
