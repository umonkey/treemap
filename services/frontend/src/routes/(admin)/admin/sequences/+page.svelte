<script lang="ts">
	import { untrack } from 'svelte';
	import { pageState } from './page.svelte.ts';
	import { formatDateTime } from '$lib/utils/strings';

	$effect(() => {
		untrack(() => pageState.reload());
	});
</script>

<svelte:head>
	<title>StreetView Sequences</title>
</svelte:head>

<article>
	<header>
		<h1>StreetView Sequences</h1>
	</header>
	{#if pageState.error}
		<p class="error">Error loading sequences: {pageState.error.description}</p>
	{/if}

	{#if pageState.isLoading}
		<p aria-busy="true">Loading sequences...</p>
	{:else}
		<div class="sequence-list">
			<table>
				<thead>
					<tr>
						<th>Captured At</th>
						<th>Images</th>
					</tr>
				</thead>
				<tbody>
					{#each pageState.sequences as seq (seq.id)}
						<tr>
							<td>{formatDateTime(seq.captured_at)}</td>
							<td>{seq.image_count}</td>
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

	.sequence-list table {
		width: 100%;
		border-collapse: collapse;
	}

	.sequence-list th,
	.sequence-list td {
		padding: 8px;
		border-bottom: 1px solid light-dark(#ddd, #444);
		text-align: left;
	}
</style>
