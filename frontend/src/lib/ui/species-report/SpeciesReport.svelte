<script lang="ts">
	import { formatNumber, pc } from './hooks';
	import { routes } from '$lib/routes';

	type Props = {
		data: {
			species: string;
			count: number;
			height: number;
			diameter: number;
			girth: number;
		}[];
		street: string;
	};

	const { data, street }: Props = $props();

	const total = data.reduce((sum, { count }) => sum + count, 0);
</script>

{#if data}
	<h3>Trees by species</h3>

	<table>
		<thead>
			<tr>
				<th>Species</th>
				<th>Count</th>
				<th>%</th>
				<th>Avg. Height, m</th>
				<th>Avg. Crown ø, m</th>
				<th>Avg. Girth, cm</th>
			</tr>
		</thead>
		<tbody>
			{#each data as { species, count, height, diameter, girth }}
				<tr>
					<td
						><a href={routes.searchQuery(`addr:"${street}" species:"${species}"`)}>{species}</a></td
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
{/if}

<style>
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
