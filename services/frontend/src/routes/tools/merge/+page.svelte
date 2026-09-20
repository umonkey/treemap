<script lang="ts">
	import { untrack } from 'svelte';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
	import { routes } from '$lib/routes';
	import { PageState } from './page.svelte.ts';

	const pageState = new PageState();

	$effect(() => {
		const cleanup = pageState.setup();
		untrack(() => pageState.reload());
		return cleanup;
	});
</script>

<svelte:head>
	<title>Merge duplicate trees</title>
</svelte:head>

<AuthWrapper>
	<div class="merge">
		<h1>Merge duplicate trees</h1>

		{#if pageState.loading}
			<p>Checking...</p>
		{:else if pageState.error}
			<p>{pageState.error.description}</p>
		{:else if pageState.data && pageState.data.duplicates.length > 0}
			<p>
				This page lists trees that have been identified as duplicates, located within a meter of
				each other.
			</p>
			<p>
				Please enrich the target tree with data from the duplicate, then remove it (mark as gone).
			</p>

			<table>
				<thead>
					<tr>
						<th>From</th>
						<th>To</th>
					</tr>
				</thead>
				<tbody>
					{#each pageState.data.duplicates as dup}
						<tr>
							<td><a href={routes.mapPreview(dup.from_id)}>{dup.from_id}</a></td>
							<td><a href={routes.mapPreview(dup.to_id)}>{dup.to_id}</a></td>
						</tr>
					{/each}
				</tbody>
			</table>
		{:else}
			<p>Congratulations, there are no duplicate trees!</p>
		{/if}
	</div>
</AuthWrapper>

<style>
	.merge {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	h1 {
		margin: 0;
		font-size: 1.5rem;
		font-weight: 500;
	}

	table {
		font-family: monospace;
	}
</style>
