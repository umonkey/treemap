<script lang="ts">
	import { locale } from '$lib/locale';
	import {
		Button,
		Buttons,
		CanopyInput,
		CircumferenceInput,
		HeightInput,
		LocationInput,
		NotesInput,
		StateInput,
		YearInput,
		SpeciesInput
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
		handleYearChange
	} = hooks();

	$effect(() => reload(id));
</script>

<div class="form">
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
		<NotesInput value={$updated.notes} onChange={handleNotesChange} />

		<Buttons>
			<Button type="submit" label={locale.editSave()} onClick={handleConfirm} disabled={$saving} />
			<Button type="cancel" label={locale.editCancel()} onClick={handleCancel} disabled={$saving} />
		</Buttons>
	{/if}
</div>
