<script lang="ts">
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import { locale } from '$lib/locale';
	import { formatSize } from '$lib/utils/strings';
	import { pageState } from './page.svelte';

	$effect(() => {
		pageState.reload();
	});
</script>

<svelte:head>
	<title>Data — {locale.appTitle()}</title>
</svelte:head>

<Dialog title="Download the database">
	<div>
		<p>
			You can download the whole database (anonymized) as a single SQLite file and do whatever you
			want with it.
		</p>
		<p>
			This data is available under the <a
				href="https://opendatacommons.org/licenses/odbl/1-0/"
				target="_blank"
				rel="noopener noreferrer">Open Database License (ODbL) v1.0</a
			>. Please attribute "Trees of Yerevan" and "OpenStreetMap contributors". Any derived database
			must be shared under the same license.
		</p>
	</div>

	{#if pageState.loading}
		<p>Loading...</p>
	{:else if pageState.error}
		<p>Error: {pageState.error.description}</p>
	{:else if pageState.data.length === 0}
		<p>No exports available.</p>
	{:else}
		<table>
			<thead>
				<tr>
					<th class="l">File</th>
					<th class="r">Size</th>
				</tr>
			</thead>
			<tbody>
				{#each pageState.data as file}
					<tr>
						<td class="l"><a href={file.url} download>{file.name}</a></td>
						<td class="r">{formatSize(file.size)}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	{/if}
</Dialog>

<style>
	table {
		line-height: 1.5;
		width: 100%;
		border-collapse: collapse;
	}

	th,
	td {
		border: 1px solid rgba(128, 128, 128, 0.5);
		padding: 0.5rem 1rem;
	}

	th.l,
	td.l {
		text-align: left;
	}

	th.r,
	td.r {
		text-align: right;
	}

	tr:hover {
		background-color: rgba(128, 128, 128, 0.1);
	}
</style>
