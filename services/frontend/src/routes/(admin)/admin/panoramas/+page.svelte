<script lang="ts">
	import { untrack } from 'svelte';
	import { PageState } from './page.svelte.ts';
	import { formatDate } from '$lib/utils/strings';
	import { storage_cost } from '$lib/utils/files';
	import Breadcrumbs from '$lib/components/admin/Breadcrumbs.svelte';
	import PageHeader from '$lib/ui/header/PageHeader.svelte';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
	import CheckInput from '$lib/ui/check-input/CheckInput.svelte';
	import { hasPermission } from '$lib/stores/authStore';

	const pageState = new PageState();

	const canEdit = $derived($hasPermission('pano:edit'));

	const totalBytes = $derived(pageState.panoramas.reduce((acc, p) => acc + (p.file_size ?? 0), 0));

	const statusLabels: Record<string, string> = {
		NEEDS_PROCESSING_FINISH: 'processing',
		SUCCESS: 'ready'
	};

	function getStatusLabel(status: string): string {
		return statusLabels[status] ?? status.toLowerCase();
	}

	$effect(() => {
		const cleanup = pageState.setup();
		untrack(() => pageState.reload());
		return cleanup;
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
							<th class="col-size">Size, GB</th>
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
								<td class="col-size">{Math.round((pano.file_size ?? 0) / (1024 * 1024 * 1024))}</td>
								<td class="col-status">{getStatusLabel(pano.status)}</td>
							</tr>
						{/each}
					</tbody>
				</table>
				<div class="panorama-summary">
					Total file size: {Math.round(totalBytes / (1024 * 1024 * 1024))} GB ~= {storage_cost(
						totalBytes
					)}/mo
				</div>
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

	td {
		text-align: left;
	}

	.panorama-list th,
	.panorama-list td {
		padding: 8px;
		border-bottom: 1px solid light-dark(#ddd, #444);
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

	.col-images,
	.col-size {
		white-space: nowrap;
		text-align: right;
	}

	.panorama-summary {
		margin-top: 1rem;
		font-weight: bold;
	}

	.col-status,
	.col-visible {
		white-space: nowrap;
	}
</style>
