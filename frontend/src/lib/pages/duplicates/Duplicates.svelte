<script lang="ts">
	import { hooks } from './hooks';
	import { routes } from '$lib/routes';

	const { data, loading } = hooks();
</script>

<h1>Duplicate trees</h1>

{#if $loading}
	<p>Checking...</p>
{:else if $data?.duplicates?.length > 0}
	<p>
		This page lists trees that have been identified as duplicates, sharing the same coordinates.
	</p>
	<p>
		Please enrich the first tree with data from the duplicates, then remove them (mark as gone).
	</p>

	<dl>
		{#each $data.duplicates as dup}
			<dt>{dup.lat}, {dup.lon}</dt>
			<dd>
				<ol>
					{#each dup.tree_ids as id}
						<li>
							<a href={routes.treeDetails(id)}>{id}</a>
						</li>
					{/each}
				</ol>
			</dd>
		{/each}
	</dl>
{:else}
	<p>Congratulations, there are no duplicate trees!</p>
{/if}

<style>
	dl {
		font-family: monospace;
	}
</style>
