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
			<Button type="submit" label={locale.addConfirmButton()} onClick={save} disabled={$busy} />
			<Button type="cancel" label={locale.addCancelButton()} onClick={close} disabled={$busy} />
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
