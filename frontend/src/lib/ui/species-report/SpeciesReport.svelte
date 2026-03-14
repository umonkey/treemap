<script lang="ts">
	import { routes } from '$lib/routes';
	import { type Record, reportState, formatNumber, pc } from './SpeciesReport.svelte.ts';

	type Props = {
		data: Record[];
		street: string;
		total: number;
	};

	const { data, street, total }: Props = $props();

	$effect(() => {
		reportState.reload(street, total, data);
	});
</script>

{#if data}
	<h3>Trees by species</h3>

	<div class="wrapper">
		<table>
			<thead>
				<tr>
					<th onclick={reportState.sortSpecies}>Species</th>
					<th onclick={reportState.sortCount}>Count</th>
					<th>%</th>
					<th onclick={reportState.sortHeight}>Avg. Height, m</th>
					<th onclick={reportState.sortCrown}>Avg. Crown ø, m</th>
					<th onclick={reportState.sortGirth}>Avg. Girth, cm</th>
				</tr>
			</thead>
			<tbody>
				{#each reportState.records as { species, count, height, diameter, girth } (species)}
					<tr>
						<td
							><a href={routes.searchQuery(`addr:"${street}" species:"${species}"`)}>{species}</a
							></td
						>
						<td>{count}</td>
						<td>{pc(count, total)}</td>
						<td>{formatNumber(height)}</td>
						<td>{formatNumber(diameter)}</td>
						<td>{Math.round(girth * 100)}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
{/if}

<style>
	.wrapper {
		width: 100%;
		overflow-x: scroll;
	}

	table {
		border-collapse: collapse;
	}

	tr {
		border-bottom: 1px solid var(--sep-color);
	}

	th {
		font-weight: normal;
		opacity: 0.5;
	}

	th,
	td {
		text-align: right;
		padding: 4px 0 4px 20px;
		vertical-align: top;
	}

	th:first-child,
	td:first-child {
		text-align: left;
		width: 100%;
		padding-left: 0;
	}

	h3 {
		margin-top: 50px;
	}
</style>
