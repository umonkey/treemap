<script lang="ts">
	import { page } from '$app/state';
	import { formatSize } from '$lib/utils/strings';
	import { locale } from '../lang';
	import { PageState } from './page.svelte.ts';

	const id = $derived(page.params.id as string);
	const pageState = new PageState();

	$effect(() => {
		pageState.reload(id);
	});
</script>

<article>
	{#if pageState.isLoading}
		<p aria-busy="true">{locale.loading()}</p>
	{:else if pageState.error}
		<p class="error" role="alert">{locale.error(pageState.error.description)}</p>
	{:else if pageState.sortedFiles.length > 0}
		<table>
			<thead>
				<tr>
					<th>{locale.name()}</th>
					<th class="col-size">{locale.size()}</th>
				</tr>
			</thead>
			<tbody>
				{#each pageState.sortedFiles as file (file.name)}
					<tr>
						<td>
							{#if file.url}
								<a href={file.url} target="_blank" rel="noopener noreferrer">{file.name}</a>
							{:else}
								{file.name}
							{/if}
						</td>
						<td class="col-size">{formatSize(file.size)}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	{:else}
		<p>{locale.noFiles()}</p>
	{/if}
</article>

<style>
	article {
		margin-top: 1rem;
	}

	table {
		width: 100%;
		border-collapse: collapse;
		font-family: monospace;
	}

	th,
	td {
		padding: 0.5rem;
		border-bottom: 1px solid light-dark(#ddd, #444);
		text-align: left;
		vertical-align: top;
	}

	th {
		font-size: 0.85rem;
		font-weight: 600;
		opacity: 0.7;
		text-transform: uppercase;
	}

	.col-size {
		white-space: nowrap;
		text-align: right;
	}

	.error {
		color: red;
	}
</style>
