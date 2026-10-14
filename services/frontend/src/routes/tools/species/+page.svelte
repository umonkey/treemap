<script lang="ts">
	import PageHeader from '$lib/ui/header/PageHeader.svelte';
	import { PageState } from './page.svelte';

	const pageState = new PageState();

	$effect(() => {
		pageState.reload();
	});
</script>

<svelte:head>
	<title>Species List</title>
</svelte:head>

<article>
	<PageHeader text="Species List" button={{ label: 'Add', link: '/tools/species/add' }} />
	{#if pageState.loading}
		<p>Loading...</p>
	{:else if pageState.error}
		<p class="error">Error loading species: {pageState.error.description}</p>
	{:else}
		<div class="species-list">
			<table>
				<thead>
					<tr>
						<th>Name</th>
						<th class="qty">Qty</th>
						<th class="wikidata">Wikidata</th>
					</tr>
				</thead>
				<tbody>
					{#each pageState.species as species (species.name)}
						<tr>
							<td>
								<a href="/tools/species/{encodeURIComponent(species.name)}">{species.name}</a>
								<div class="local">{species.local}</div>
							</td>
							<td class="qty">{species.count}</td>
							<td class="wikidata">
								{#if species.wikidata_id}
									<a
										href={`https://www.wikidata.org/wiki/${encodeURIComponent(species.wikidata_id)}`}
										target="_blank"
										rel="noreferrer"
									>
										{species.wikidata_id}
									</a>
								{:else}
									—
								{/if}
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</article>

<style>
	.error {
		color: red;
	}

	.species-list table {
		width: 100%;
		border-collapse: collapse;
	}

	.species-list th,
	.species-list td {
		padding: 8px;
		border-bottom: 1px solid light-dark(#ddd, #444);
		text-align: left;
	}

	.species-list .local {
		opacity: 0.8;
	}

	.species-list .qty {
		text-align: right;
	}

	.species-list .wikidata {
		text-align: center;
	}
</style>
