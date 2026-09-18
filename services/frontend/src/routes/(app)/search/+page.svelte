<script lang="ts">
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import ToolSelector from '$lib/components/search/ToolSelector.svelte';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import SearchBar from './SearchBar.svelte';

	import SpeciesInput from '$lib/ui/species-input/SpeciesInput.svelte';
	import StreetInput from '$lib/ui/street-input/StreetInput.svelte';
	import AgeInput from './AgeInput.svelte';
	import StateInput from '$lib/ui/state-input/StateInput.svelte';
	import { pageState } from './page.svelte';

	const buttons = [
		{
			title: 'Search',
			onClick: pageState.handleSubmit
		}
	];
</script>

<svelte:head>
	<title>{locale.sideSearch()} — {locale.appTitle()}</title>
</svelte:head>

<Dialog title={locale.searchTitle()} {buttons}>
	<SearchBar
		value={pageState.query}
		onInput={pageState.handleInput}
		onSearch={pageState.handleSearch}
	/>

	<div>
		{#if pageState.query}
			<p>
				<a href={routes.searchQuery(pageState.query)} data-testid="search-link"
					>{locale.searchLink(pageState.query)}</a
				>
			</p>
			<hr />
		{/if}
	</div>

	<div class="form">
		<StateInput value={pageState.state} onChange={pageState.handleStateChange} />

		<SpeciesInput
			value={pageState.species}
			onChange={pageState.handleSpeciesChange}
			nosuggestions={true}
		/>

		<StreetInput value={pageState.street} onChange={pageState.handleStreetChange} />

		<ToolSelector />

		<AgeInput value={pageState.age} onChange={pageState.handleAgeChange} />
	</div>
</Dialog>

<style>
	hr {
		border: none;
		border-top: 1px solid var(--sep-color);
		height: 0;
		margin: var(--gap) 0;
	}

	.form {
		display: flex;
		flex-direction: column;
		gap: calc(2 * var(--gap));
	}
</style>
