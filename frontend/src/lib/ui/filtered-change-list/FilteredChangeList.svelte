<script lang="ts">
	import type { IChange } from '$lib/types';
	import { format } from './hooks';
	import { locale } from '$lib/locale';

	const { changes, name } = $props<{
		changes: IChange[];
		name: string;
	}>();

	let formatted = $state([]);

	$effect(() => {
		formatted = format(changes, name);
	});

	const className = `change-list ${name}`;
</script>

<div class={className}>
	{#if formatted.length > 0}
		<h2>Recent changes:</h2>
		<table>
			<tbody>
				{#each formatted as change}
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
	.change-list {
		margin: calc(var(--gap) * 3) 0 0;
	}

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
