<script lang="ts">
	import PageHeader from '$lib/ui/header/PageHeader.svelte';
	import { routes } from '$lib/routes';
	import { PageState } from './page.svelte';

	const pageState = new PageState();

	$effect(() => {
		pageState.reload();
	});
</script>

<svelte:head>
	<title>Trees by Status</title>
</svelte:head>

<article>
	<PageHeader text="Trees by Status" />
	{#if pageState.loading}
		<p>Loading...</p>
	{:else if pageState.error}
		<p>{pageState.error.description}</p>
	{:else}
		<table>
			<thead>
				<tr>
					<th class="l">State</th>
					<th class="r">Count</th>
				</tr>
			</thead>
			<tbody>
				{#each pageState.data as { state, count }}
					<tr>
						{#if state}
							<td class="l"
								><a href={routes.searchState(state)}>{state ? state : '(not set)'}</a></td
							>
						{:else}
							<td class="l">not set</td>
						{/if}
						<td class="r">{count}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	{/if}
</article>

<style>
	table {
		line-height: 1.5;
		width: 100%;
	}

	th.l,
	td.l {
		text-align: left;
	}

	th.r,
	td.r {
		text-align: right;
	}
</style>
