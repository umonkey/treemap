<script lang="ts">
	/**
	 * This form lets the user fill in details for the new tree,
	 * sends the tree to the API, then reports success to the container.
	 */

	import { locale } from '$lib/locale';
	import { editor } from './hooks';
	import {
		Button,
		Buttons,
		CanopyInput,
		CircumferenceInput,
		HeightInput,
		NotesInput,
		SpeciesInput,
		StateInput,
		YearInput,
		Form
	} from '$lib/ui';
	import Title from '$lib/components/tree/Title.svelte';
	import TreeContextMenu from '$lib/components/tree/TreeContextMenu.svelte';

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
{:else}
	<Title title={$tree?.species} address={$tree?.address} />
	<TreeContextMenu id={$tree.id} />

	<Form>
		<p>{locale.replaceHint()}</p>
		<hr />

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

		<Buttons>
			<Button type="submit" onClick={save} disabled={$busy}>{locale.addConfirmButton()}</Button>
			<Button type="cancel" onClick={close} disabled={$busy}>{locale.addCancelButton()}</Button>
		</Buttons>

		{#if $saveError}
			<p>{$saveError}</p>
		{/if}
	</Form>
{/if}

<style>
	hr {
		border: 0;
		border-top: 1px solid var(--sep-color);
		margin: 1em 0;
	}
</style>
