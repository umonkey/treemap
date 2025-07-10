<script lang="ts">
	import { locale } from '$lib/locale';
	import {
		AddressInput,
		Button,
		Buttons,
		CanopyInput,
		CircumferenceInput,
		HeightInput,
		LocationInput,
		NotesInput,
		StateInput,
		YearInput,
		SpeciesInput,
		Form
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

<Form>
	{#if $loading}
		<!-- Loading... -->
	{:else}
		<SpeciesInput value={$updated.species} onChange={handleSpeciesChange} />
		<HeightInput value={$updated.height} onChange={handleHeightChange} />
		<CanopyInput value={$updated.diameter} onChange={handleDiameterChange} />
		<CircumferenceInput value={$updated.circumference} onChange={handleCircumferenceChange} />
		<StateInput value={$updated.state} onChange={handleStateChange} />
		<YearInput value={$updated.year} onChange={handleYearChange} />
		<LocationInput
			value={{ lat: $tree.lat, lng: $tree.lon }}
			pin={{ lat: $tree.lat, lng: $tree.lon }}
			onChange={handleLocationChange}
		/>

		<AddressInput value={$tree.address} onChange={handleAddressChange} />

		<NotesInput value={$updated.notes} onChange={handleNotesChange} />

		<Buttons>
			<Button type="submit" onClick={handleConfirm} disabled={$saving}>{locale.editSave()}</Button>
			<Button type="cancel" onClick={handleCancel} disabled={$saving}>{locale.editCancel()}</Button>
		</Buttons>
	{/if}
</Form>
