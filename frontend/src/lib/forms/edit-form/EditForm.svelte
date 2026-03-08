<script lang="ts">
	import Title from '$lib/components/tree/Title.svelte';
	import TreeContextMenu from '$lib/components/tree/TreeContextMenu.svelte';
	import { locale } from '$lib/locale';
	import {
		Button,
		Buttons,
		CanopyInput,
		CircumferenceInput,
		Form,
		HeightInput,
		LocationInput,
		NotesInput,
		SpeciesInput,
		StateInput,
		StreetInput,
		YearInput
	} from '$lib/ui';
	import { hooks } from './hooks';

	const { id } = $props<{
		id: string;
	}>();

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

	$effect(() => reload(id));
</script>

{#if $loading}
	<!-- Loading... -->
{:else if $tree}
	<Title title={$tree.species} address={$tree.address ?? undefined} />
	<TreeContextMenu id={$tree.id} />

	<Form onSubmit={handleConfirm} sticky>
		<SpeciesInput value={$updated.species} onChange={handleSpeciesChange} />
		<HeightInput value={$updated.height} onChange={handleHeightChange} />
		<CanopyInput value={$updated.diameter} onChange={handleDiameterChange} />
		<CircumferenceInput value={$updated.circumference} onChange={handleCircumferenceChange} />
		<StateInput value={$updated.state} onChange={handleStateChange} />
		<YearInput value={$updated.year} onChange={handleYearChange} />
		<LocationInput value={{ lat: $tree.lat, lng: $tree.lon }} onChange={handleLocationChange} />

		<StreetInput value={$tree.address} onChange={handleAddressChange} />

		<NotesInput value={$updated.notes} onChange={handleNotesChange} />

		<Buttons sticky>
			<Button type="submit" onClick={handleConfirm} disabled={$saving}>{locale.editSave()}</Button>
			<Button type="cancel" onClick={handleCancel} disabled={$saving}>{locale.editCancel()}</Button>
		</Buttons>
	</Form>
{/if}
