<script lang="ts">
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import SearchBar from './SearchBar.svelte';

	import SpeciesInput from '$lib/ui/species-input/SpeciesInput.svelte';
	import StreetInput from '$lib/ui/street-input/StreetInput.svelte';
	import AgeInput from './AgeInput.svelte';
	import StateInput from '$lib/ui/state-input/StateInput.svelte';
	import CheckInput from '$lib/ui/check-input/CheckInput.svelte';
	import { pageState } from './page.svelte';

	const buttons = [
		{
			title: 'Search',
			onClick: pageState.handleSubmit
		}
	];
</script>

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
		<div class="checks">
			<CheckInput
				label={locale.searchMissingHeight()}
				value={pageState.noHeight}
				onChange={pageState.handleNoHeightChange}
			/>
			<CheckInput
				label={locale.searchMissingCanopy()}
				value={pageState.noCanopy}
				onChange={pageState.handleNoCanopyChange}
			/>
			<CheckInput
				label={locale.searchMissingCircumference()}
				value={pageState.noCircumference}
				onChange={pageState.handleNoCircumferenceChange}
			/>
			<CheckInput
				label={locale.searchMissingObservations()}
				value={pageState.noObservations}
				onChange={pageState.handleNoObservationsChange}
			/>
			<CheckInput
				label={locale.searchMissingPhotos()}
				value={pageState.noPhotos}
				onChange={pageState.handleNoPhotosChange}
			/>
		</div>

		<StateInput value={pageState.state} onChange={pageState.handleStateChange} />

		<SpeciesInput
			value={pageState.species}
			onChange={pageState.handleSpeciesChange}
			nosuggestions={true}
		/>
		<StreetInput value={pageState.street} onChange={pageState.handleStreetChange} />
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

	.checks {
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: var(--gap);
	}

	@media (max-width: 600px) {
		.checks {
			grid-template-columns: 1fr;
		}
	}
</style>
