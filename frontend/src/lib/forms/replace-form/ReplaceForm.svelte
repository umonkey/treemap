<script lang="ts">
	/**
	 * This form lets the user fill in details for the new tree,
	 * sends the tree to the API, then reports success to the container.
	 */

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
		NotesInput,
		SpeciesInput,
		StateInput,
		YearInput
	} from '$lib/ui';
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
	<Title title={$tree.species} address={$tree.address ?? undefined} />
	<TreeContextMenu id={$tree.id} />

	<Form sticky>
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

		<Buttons sticky>
			<Button type="submit" onClick={save} disabled={$busy}>{locale.addConfirmButton()}</Button>
			<Button type="cancel" onClick={close} disabled={$busy}>{locale.addCancelButton()}</Button>
		</Buttons>

		{#if $saveError}
			<p>{$saveError}</p>
		{/if}
	</Form>
{/if}
