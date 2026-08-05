<script lang="ts">
	import { untrack } from 'svelte';
	import { PageState } from './page.svelte.ts';
	import { page } from '$app/state';
	import Breadcrumbs from '$lib/components/admin/Breadcrumbs.svelte';
	import PanoramaHeader from '$lib/components/panoramas/PanoramaHeader.svelte';
	import PageHeader from '$lib/ui/header/PageHeader.svelte';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import VideoUploader from './VideoUploader.svelte';
	import TrackUploader from './TrackUploader.svelte';
	import VideoSync from './VideoSync.svelte';
	import PanoramaPreview from './PanoramaPreview.svelte';

	const id = $derived(page.params.id as string);
	const pageState = new PageState();

	$effect(() => {
		untrack(() => pageState.reload(id));
	});
</script>

<svelte:head>
	<title>Panorama: {pageState.panorama?.title || id}</title>
</svelte:head>

<AuthWrapper permission="pano:edit">
	<PageHeader
		text={pageState.panorama?.title || id}
		button={{ label: 'Edit', link: `/admin/panoramas/${id}/edit` }}
	/>
	<Breadcrumbs
		items={[
			{ label: 'Admin', href: '/admin' },
			{ label: 'Panoramas', href: '/admin/panoramas' },
			{ label: pageState.panorama?.title || id }
		]}
	/>

	{#if pageState.panorama}
		<PanoramaHeader
			{id}
			title={pageState.panorama.title}
			createdAt={pageState.panorama.created_at}
			status={pageState.panorama.status}
			bind:visible={pageState.panorama.visible}
		/>
	{/if}

	<article>
		{#if pageState.error}
			<p class="error">Error loading panorama: {pageState.error.description}</p>
		{:else if pageState.isLoading}
			<p aria-busy="true">Loading panorama...</p>
		{:else if pageState.panorama}
			<div class="panorama-details">
				<table>
					<tbody>
						{#if pageState.panorama.failure_reason}
							<tr>
								<th>Failure Reason</th>
								<td class="error">{pageState.panorama.failure_reason}</td>
							</tr>
						{/if}

						<tr>
							<th>Images:</th>
							<td>{pageState.panorama.image_count}</td>
						</tr>

						<tr>
							<th>GPS Time Offset</th>
							<td>{pageState.panorama.gpx_offset ?? 'not set'}</td>
						</tr>

						<tr>
							<th>Transcode job status</th>
							<td>{pageState.panorama.transcode_status ?? 'unknown'}</td>
						</tr>

						<tr>
							<th>Processing job status</th>
							<td>{pageState.panorama.processing_status ?? 'unknown'}</td>
						</tr>
					</tbody>
				</table>
			</div>

			{#if !pageState.panorama.source_video_path}
				<VideoUploader panoramaId={id} onUploadSuccess={() => pageState.reload(id)} />
			{:else if !pageState.panorama.gpx_path}
				<TrackUploader panoramaId={id} onUploadSuccess={() => pageState.reload(id)} />
			{:else if pageState.panorama.status === 'FAILURE'}
				<p class="error">
					We could not process the uploaded file: {pageState.panorama.failure_reason ||
						'Please contact technical support.'}
				</p>
			{:else if pageState.panorama.status === 'NEEDS_TRANSCODING' || pageState.panorama.status === 'NEEDS_TRANSCODING_FINISH'}
				<div class="message">
					<p>We are processing the uploaded video file, please wait.</p>
					<p>You will get an email when we're ready for next steps.</p>
				</div>
			{:else if pageState.panorama.status === 'NEEDS_SYNC'}
				<VideoSync panoramaId={id} {pageState} />
			{:else if pageState.panorama.status === 'NEEDS_PROCESSING' || pageState.panorama.status === 'NEEDS_PROCESSING_FINISH'}
				<p>Processing panorama data, please wait...</p>
			{:else if pageState.panorama.status === 'SUCCESS'}
				<PanoramaPreview panoramaId={id} />
			{/if}

			{#if pageState.panorama.status === 'SUCCESS' || pageState.panorama.status === 'FAILURE'}
				<Buttons>
					<Button type="secondary" onClick={() => pageState.exportData(id)}>Download data</Button>
					<Button type="secondary" link="/admin/panoramas/{id}/restart">Restart Panorama</Button>
				</Buttons>
			{/if}
		{/if}
	</article>
</AuthWrapper>

<style>
	.error {
		color: red;
	}

	article {
		display: flex;
		flex-direction: column;
		gap: 1rem;
		margin-top: 1rem;
	}

	.panorama-details table {
		width: 100%;
		border-collapse: collapse;
		margin-bottom: 2rem;
	}

	.panorama-details th,
	.panorama-details td {
		padding: 0.5rem 1rem;
		border-bottom: 1px solid light-dark(#ddd, #444);
		text-align: left;
		vertical-align: top;
	}

	.panorama-details th {
		width: 250px;
		font-weight: bold;
	}
</style>
