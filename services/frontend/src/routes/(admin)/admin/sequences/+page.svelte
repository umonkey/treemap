<script lang="ts">
	import { untrack } from 'svelte';
	import { pageState } from './page.svelte.ts';
	import { formatDateTimeISO } from '$lib/utils/strings';
	import Breadcrumbs from '$lib/components/admin/Breadcrumbs.svelte';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';

	$effect(() => {
		untrack(() => pageState.reload());
	});
</script>

<svelte:head>
	<title>StreetView Sequences</title>
</svelte:head>

<AuthWrapper permission="pano:edit">
	<article>
		<header>
			<h1>StreetView Sequences</h1>

			<Breadcrumbs items={[{ label: 'Admin', href: '/admin' }, { label: 'Sequences' }]} />
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
							<th class="col-date">Captured At</th>
							<th class="col-title">Title</th>
							<th class="col-images">Images</th>
							<th class="col-status">Status</th>
						</tr>
					</thead>
					<tbody>
						{#each pageState.sequences as seq (seq.id)}
							<tr>
								<td class="col-date">{formatDateTimeISO(seq.captured_at)}</td>
								<td>
									<a href="/admin/sequences/{seq.id}">{seq.title}</a>
								</td>
								<td class="col-images">{seq.image_count}</td>
								<td class="col-status">{seq.hidden ? 'hidden' : ''}</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}
	</article>
</AuthWrapper>

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

	.col-date {
		white-space: nowrap;
	}

	.col-title {
		width: 100%;
	}

	.col-images,
	.col-status {
		white-space: nowrap;
	}
</style>
