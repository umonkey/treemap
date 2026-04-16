<script lang="ts">
	/**
	 * This form lets the user fill in details for the new tree,
	 * sends the tree to the API, then reports success to the container.
	 */

	import TreeForm from '$lib/components/forms/TreeForm.svelte';
	import { locale } from '$lib/locale';
	import CanopyInput from '$lib/ui/canopy-input/CanopyInput.svelte';
	import CircumferenceInput from '$lib/ui/circumference-input/CircumferenceInput.svelte';
	import HeightInput from '$lib/ui/height-input/HeightInput.svelte';
	import NotesInput from '$lib/ui/notes-input/NotesInput.svelte';
	import SpeciesInput from '$lib/ui/species-input/SpeciesInput.svelte';
	import StateInput from '$lib/ui/state-input/StateInput.svelte';
	import YearInput from '$lib/ui/year-input/YearInput.svelte';
	import { editor } from './hooks';

	const { id } = $props<{
		id: string;
	}>();

	const {
		loading,
		busy,
		loadError,
		saveError,
		species,
		height,
		diameter,
		circumference,
		currentState,
		tree,
		year,
		notes,
		save,
		close
	} = editor(id);
</script>

{#if $loading}
	<!-- loading -->
{:else if $loadError}
	<p>{$loadError}</p>
{:else if $tree}
	<TreeForm {id} title="Replace Tree" onSubmit={save} onCancel={close} saving={$busy}>
		<p>{locale.replaceHint()}</p>

		<SpeciesInput value={$species} onChange={(value: string) => species.set(value)} />
		<HeightInput value={$height} onChange={(value: number | null) => height.set(value)} />
		<CanopyInput value={$diameter} onChange={(value: number | null) => diameter.set(value)} />
		<CircumferenceInput
			value={$circumference}
			onChange={(value: number) => circumference.set(value)}
		/>
		<StateInput value={$currentState} onChange={(value: string) => currentState.set(value)} />
		<YearInput value={$year} onChange={(value: number) => year.set(value)} />
		<NotesInput value={$notes} onChange={(value: string) => notes.set(value)} />

		{#if $saveError}
			<p>{$saveError}</p>
		{/if}
	</TreeForm>
{/if}
