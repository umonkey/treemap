<script lang="ts">
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import { routes } from '$lib/routes';
	import { pageState } from './page.svelte';

	$effect(() => {
		pageState.reload();
	});
</script>

<Dialog title="Trees by Species">
	{#if pageState.loading}
		<p>Loading...</p>
	{:else if pageState.error}
		<p>Error: {pageState.error.description}</p>
	{:else}
		<table>
			<thead>
				<tr>
					<th class="l" onclick={() => pageState.reorder('name')}>Species</th>
					<th class="r" onclick={() => pageState.reorder('count')}>Count</th>
				</tr>
			</thead>
			<tbody>
				{#each pageState.data as { name, count, subspecies }}
					<tr class="genus">
						<td class="l name"><a href={routes.searchSpecies(name)}>{name}</a></td>
						<td class="r">{count}</td>
					</tr>

					{#each subspecies as { name, count }}
						<tr class="species">
							<td class="l name"><a href={routes.searchSpecies(name)}>{name}</a></td>
							<td class="r">{count}</td>
						</tr>
					{/each}
				{/each}
			</tbody>
		</table>
	{/if}
</Dialog>

<style>
	table {
		line-height: 1.5;
		width: 100%;
		border-collapse: collapse;
	}

	th.l,
	td.l {
		text-align: left;
	}

	th.r,
	td.r {
		text-align: right;
	}

	tr.genus {
		margin-top: 10px;
	}

	tr.species {
		opacity: 0.8;

		& > td.name {
			padding-left: 20px;
		}
	}

	tr:hover {
		background-color: rgba(128, 128, 128, 0.1);
	}
</style>
