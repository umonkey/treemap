<script lang="ts">
	import { load, format } from './ChangeHistory.ts';
	import { locale } from '$lib/locale';

	const { id, name } = $props<{
		id: string;
		name: string;
	}>();

	const { loading, error, history } = load(id, name);

	const className = `change-list ${name}`;
</script>

<div class={className}>
	{#if $loading}
		<p>Loading change history...</p>
	{:else if $error}
		<p>Error loading change history.</p>
	{:else if $history.length > 0}
		<h2>Recent changes:</h2>
		<table>
			<tbody>
				{#each format($history) as change}
					<tr>
						<td>{change.date}</td>
						<td>{change.value}</td>
						<td>{change.author}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	{:else}
		<p>{locale.noChangeHistory()}</p>
	{/if}
</div>

<style>
	h2 {
		font-size: 125%;
		font-weight: 400;
		margin: var(--gap) 0;
	}

	table {
		display: table;
		border-spacing: 0;

		td {
			padding: 5px 20px 5px 0;
		}
	}

	.change-list.height td:nth-child(2) {
		text-align: right;
	}
</style>
