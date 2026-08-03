<script lang="ts">
	import { untrack } from 'svelte';
	import { pageState } from './page.svelte.ts';
	import { formatDate } from '$lib/utils/strings';
	import Breadcrumbs from '$lib/components/admin/Breadcrumbs.svelte';
	import PageHeader from '$lib/ui/header/PageHeader.svelte';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
	import CheckInput from '$lib/ui/check-input/CheckInput.svelte';
	import { hasPermission } from '$lib/stores/authStore';

	const canEdit = $derived($hasPermission('pano:edit'));

	const statusLabels: Record<string, string> = {
		NEEDS_PROCESSING_FINISH: 'processing',
		SUCCESS: 'ready',
		NEEDS_TRANSCODING_FINISH: 'transcoding'
	};

	function getStatusLabel(status: string): string {
		return statusLabels[status] ?? status.toLowerCase();
	}

	$effect(() => {
		untrack(() => pageState.reload());
	});
</script>

<svelte:head>
	<title>Manage Panoramas</title>
</svelte:head>

<AuthWrapper permission="pano:edit">
	<article>
		<PageHeader text="Manage Panoramas" button={{ label: 'Add', link: '/admin/panoramas/add' }} />
		<Breadcrumbs items={[{ label: 'Admin', href: '/admin' }, { label: 'Panoramas' }]} />

		{#if pageState.error}
			<p class="error">Error loading panoramas: {pageState.error.description}</p>
		{/if}

		{#if pageState.isLoading}
			<p aria-busy="true">Loading panoramas...</p>
		{:else if pageState.panoramas.length > 0}
			<div class="panorama-list">
				<table>
					<thead>
						<tr>
							<th class="col-date">Date</th>
							<th class="col-visible">Visible</th>
							<th class="col-title">Title</th>
							<th class="col-images">Images</th>
							<th class="col-status">Status</th>
						</tr>
					</thead>
					<tbody>
						{#each pageState.panoramas as pano (pano.id)}
							<tr class:disabled={pano.status !== 'SUCCESS'}>
								<td class="col-date">{formatDate(pano.created_at)}</td>
								<td class="col-visible">
									<CheckInput
										value={pano.visible}
										disabled={pano.status !== 'SUCCESS' || !canEdit}
										onChange={(visible) => pageState.updateVisibility(pano.id, visible)}
									/>
								</td>
								<td>
									<a href="/admin/panoramas/{pano.id}">{pano.title}</a>
								</td>
								<td class="col-images">{pano.image_count}</td>
								<td class="col-status">{getStatusLabel(pano.status)}</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{:else}
			<p>No panoramas found.</p>
		{/if}
	</article>
</AuthWrapper>

<style>
	.error {
		color: red;
	}

	.panorama-list table {
		width: 100%;
		border-collapse: collapse;
	}

	.panorama-list th,
	.panorama-list td {
		padding: 8px;
		border-bottom: 1px solid light-dark(#ddd, #444);
		text-align: left;
	}

	.panorama-list tr.disabled {
		opacity: 0.5;
	}

	.col-date {
		white-space: nowrap;
	}

	.col-title {
		width: 100%;
	}

	.col-images {
		white-space: nowrap;
		text-align: right;
	}

	.col-status,
	.col-visible {
		white-space: nowrap;
	}
</style>
