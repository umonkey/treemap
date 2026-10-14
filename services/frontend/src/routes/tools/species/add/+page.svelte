<script lang="ts">
	import PageHeader from '$lib/ui/header/PageHeader.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import Form from '$lib/ui/form/Form.svelte';
	import TextInput from '$lib/ui/text-input/TextInput.svelte';
	import { PageState } from './page.svelte';

	const pageState = new PageState();
</script>

<svelte:head>
	<title>Add Species</title>
</svelte:head>

<article>
	<PageHeader text="Add Species" />

	<Form onSubmit={pageState.handleSave}>
		<TextInput
			label="Name"
			value={pageState.name}
			placeholder="Quercus robur"
			hint="The latin name. Used as the primary key, so it must be unique."
			onChange={(value) => (pageState.name = value)}
		/>

		<TextInput
			label="Local Name"
			value={pageState.local}
			placeholder="Pedunculate oak"
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
				{pageState.saving ? 'Creating...' : 'Create'}
			</Button>
			<Button link="/tools/species" type="cancel">Cancel</Button>
		</Buttons>
	</Form>
</article>
