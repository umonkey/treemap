<script lang="ts">
	import { page } from '$app/state';
	import PageHeader from '$lib/ui/header/PageHeader.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import Form from '$lib/ui/form/Form.svelte';
	import TextInput from '$lib/ui/text-input/TextInput.svelte';
	import { PageState } from './page.svelte';

	const pageState = new PageState();

	const name = $derived(decodeURIComponent(page.params.name as string));

	$effect(() => {
		if (name) {
			pageState.reload(name);
		}
	});
</script>

<svelte:head>
	{#if pageState.name}
		<title>Species Editor: {pageState.name}</title>
	{/if}
</svelte:head>

<article>
	{#if pageState.loading}
		<p>Loading...</p>
	{:else if pageState.error}
		<p class="error">Error loading species: {pageState.error.description}</p>
	{:else}
		<PageHeader text="Species Editor" />

		<Form onSubmit={pageState.handleSave}>
			<div class="field">
				<label for="name">Name</label>
				<input id="name" type="text" value={pageState.name} disabled />
			</div>

			<TextInput
				label="Local Name"
				value={pageState.local}
				onChange={(value) => (pageState.local = value)}
			/>

			<TextInput
				label="Keywords"
				value={pageState.keywords}
				hint="Separate keywords with semicolons or commas."
				onChange={(value) => (pageState.keywords = value)}
			/>

			<TextInput
				label="Wikidata ID"
				value={pageState.wikidataId}
				placeholder="Q165145"
				hint="Wikidata item id, for example Q165145. Leave empty if unknown."
				onChange={(value) => (pageState.wikidataId = value)}
			/>

			<Buttons>
				<Button type="submit" disabled={pageState.saving}>
					{pageState.saving ? 'Saving...' : 'Save'}
				</Button>
				<Button link={`/tools/species/${encodeURIComponent(pageState.name)}`} type="cancel">
					Cancel
				</Button>
			</Buttons>
		</Form>
	{/if}
</article>

<style>
	.error {
		color: red;
	}

	.field {
		margin-bottom: 15px;
	}

	.field label {
		display: block;
		margin-bottom: 5px;
		font-weight: bold;
	}

	.field input {
		width: 100%;
		padding: 8px;
		border: 1px solid var(--input-border);
		border-radius: 4px;
		box-sizing: border-box;
	}

	.field input:disabled {
		background-color: var(--disabled-input-bg);
		color: var(--disabled-input-fg);
	}
</style>
