<script lang="ts">
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import { routes } from '$lib/routes';
	import { pageState } from './page.svelte';

	$effect(() => {
		pageState.reload();
	});
</script>

<Dialog title="Duplicate Trees">
	<div>
		{#if pageState.loading}
			<p>Checking...</p>
		{:else if pageState.error}
			<p>{pageState.error.description}</p>
		{:else if pageState.data && pageState.data.duplicates.length > 0}
			<p>
				This page lists trees that have been identified as duplicates, sharing the same coordinates.
			</p>
			<p>
				Please enrich the first tree with data from the duplicates, then remove them (mark as gone).
			</p>

			<dl>
				{#each pageState.data.duplicates as dup}
					<dt>{dup.lat}, {dup.lon}</dt>
					<dd>
						<ol>
							{#each dup.tree_ids as id}
								<li>
									<a href={routes.mapPreview(id)}>{id}</a>
								</li>
							{/each}
						</ol>
					</dd>
				{/each}
			</dl>
		{:else}
			<p>Congratulations, there are no duplicate trees!</p>
		{/if}
	</div>
</Dialog>

<style>
	dl {
		font-family: monospace;
	}
</style>
