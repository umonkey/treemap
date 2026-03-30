<script lang="ts">
	import { loadSpeciesStats } from '$lib/hooks';
	import { routes } from '$lib/routes';
	import Dialog from '$lib/components/layout/Dialog.svelte';

	const { loading, error, data, reload, reorder } = loadSpeciesStats();

	$effect(() => {
		reload();
	});
</script>

<Dialog title="Trees by Species">
	{#if $loading}
		<p>Loading...</p>
	{:else if $error}
		<p>Error: {$error.description}</p>
	{:else}
		<table>
			<thead>
				<tr>
					<th class="l" onclick={() => reorder('name')}>Species</th>
					<th class="r" onclick={() => reorder('count')}>Count</th>
				</tr>
			</thead>
			<tbody>
				{#each $data as { name, count, subspecies }}
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
