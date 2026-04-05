<script lang="ts">
	import { page } from '$app/stores';
	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import { DEFAULT_MAP_CENTER } from '$lib/constants';
	import { locale } from '$lib/locale';
	import { pageState } from './page.svelte';

	import CanopyInput from '$lib/ui/canopy-input/CanopyInput.svelte';
	import CircumferenceInput from '$lib/ui/circumference-input/CircumferenceInput.svelte';
	import HeightInput from '$lib/ui/height-input/HeightInput.svelte';
	import LocationInput from '$lib/ui/location-input/LocationInput.svelte';
	import NotesInput from '$lib/ui/notes-input/NotesInput.svelte';
	import SpeciesInput from '$lib/ui/species-input/SpeciesInput.svelte';
	import StateInput from '$lib/ui/state-input/StateInput.svelte';
	import YearInput from '$lib/ui/year-input/YearInput.svelte';

	const lat = $derived(
		Number.parseFloat($page.url.searchParams.get('lat') || DEFAULT_MAP_CENTER.lat.toString())
	);
	const lng = $derived(
		Number.parseFloat($page.url.searchParams.get('lng') || DEFAULT_MAP_CENTER.lng.toString())
	);

	$effect(() => {
		pageState.reload(lat, lng);
	});
</script>

{#if pageState.loading}
	<!-- loading ... -->
{:else}
	<TreeForm
		title="Add Tree"
		onSubmit={pageState.handleConfirm}
		onCancel={pageState.handleCancel}
		saving={pageState.saving}
	>
		<LocationInput
			value={pageState.location}
			label={locale.addConfirmLocation()}
			onChange={pageState.handleLocationChange}
		/>
		<SpeciesInput value={pageState.tree.species} onChange={pageState.handleSpeciesChange} />
		<StateInput value={pageState.tree.state} onChange={pageState.handleStateChange} />
		<HeightInput value={pageState.tree.height} onChange={pageState.handleHeightChange} />
		<CanopyInput value={pageState.tree.diameter} onChange={pageState.handleDiameterChange} />
		<CircumferenceInput
			value={pageState.tree.circumference}
			onChange={pageState.handleCircumferenceChange}
		/>
		<YearInput value={pageState.tree.year} onChange={pageState.handleYearChange} />
		<NotesInput value={pageState.tree.notes} onChange={pageState.handleNotesChange} />
	</TreeForm>
{/if}
