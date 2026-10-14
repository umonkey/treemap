<script lang="ts">
	import { page } from '$app/state';
	import WikiIcon from '$lib/icons/WikipediaIcon.svelte';
	import { routes } from '$lib/routes';
	import Button from '$lib/ui/button/Button.svelte';
	import PageHeader from '$lib/ui/header/PageHeader.svelte';
	import { PageState } from './page.svelte';

	const pageState = new PageState();

	const name = $derived(decodeURIComponent(page.params.name as string));

	$effect(() => {
		if (name) {
			pageState.reload(name);
		}
	});

	const species = $derived(pageState.species);
</script>

<svelte:head>
	{#if species}
		<title>Species: {species.name}</title>
	{/if}
</svelte:head>

<article>
	{#if pageState.loading}
		<p>Loading...</p>
	{:else if pageState.error}
		<p class="error">Error loading species: {pageState.error.description}</p>
	{:else if species}
		<PageHeader
			text="Species"
			button={{ label: 'Edit', link: `/tools/species/${encodeURIComponent(species.name)}/edit` }}
		/>
		<div class="species-details">
			<h2>{species.name}</h2>

			<dl>
				<dt>Local Name</dt>
				<dd>{species.local}</dd>

				<dt>Keywords</dt>
				<dd>
					{#if species.keywords.length}
						<ul>
							{#each species.keywords as keyword}
								<li>{keyword}</li>
							{/each}
						</ul>
					{:else}
						—
					{/if}
				</dd>

				<dt>Wikidata</dt>
				<dd>
					{#if species.wikidata_id}
						<a
							href={`https://www.wikidata.org/wiki/${encodeURIComponent(species.wikidata_id)}`}
							target="_blank"
							rel="noreferrer"
						>
							<span class="wiki-icon"><WikiIcon /></span>
							{species.wikidata_id}
						</a>
					{:else}
						—
					{/if}
				</dd>

				<dt>Map</dt>
				<dd>
					<a href={routes.searchSpecies(species.name)}>Trees on the map</a>
				</dd>
			</dl>
		</div>

		<div class="actions">
			<Button link="/tools/species" type="cancel">Back to list</Button>
		</div>
	{/if}
</article>

<style>
	.error {
		color: red;
	}

	.actions {
		margin-top: 20px;
	}

	.species-details dl {
		display: grid;
		grid-template-columns: max-content auto;
		gap: 10px 20px;
	}

	.species-details dd {
		margin: 0;
	}

	.species-details ul {
		margin: 0;
		padding-left: 1.25rem;
	}

	.wiki-icon {
		display: inline-block;
		height: 16px;
		width: 16px;
		vertical-align: text-bottom;

		:global(svg) {
			height: 16px;
			width: 16px;
		}
	}
</style>
