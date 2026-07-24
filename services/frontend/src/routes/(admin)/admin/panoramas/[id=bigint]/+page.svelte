<script lang="ts">
	import { untrack } from 'svelte';
	import { pageState } from './page.svelte.ts';
	import { page } from '$app/state';
	import { formatDateTimeISO } from '$lib/utils/strings';
	import Breadcrumbs from '$lib/components/admin/Breadcrumbs.svelte';
	import AuthWrapper from '$lib/ui/auth-wrapper/AuthWrapper.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import VideoUploader from './VideoUploader.svelte';
	import TrackUploader from './TrackUploader.svelte';
	import VideoPlayer from './VideoPlayer.svelte';
	import TrackPreview from './TrackPreview.svelte';

	const id = $derived(page.params.id as string);
	let videoOffset = $state(0);

	$effect(() => {
		untrack(() => pageState.reload(id));
	});
</script>

<svelte:head>
	<title>Panorama: {pageState.panorama?.title || id}</title>
</svelte:head>

<AuthWrapper permission="pano:edit">
	<header>
		<h1>Panorama: {pageState.panorama?.title || id}</h1>
		<Breadcrumbs
			items={[
				{ label: 'Admin', href: '/admin' },
				{ label: 'Panoramas', href: '/admin/panoramas' },
				{ label: id }
			]}
		/>
	</header>

	<article>
		{#if pageState.error}
			<p class="error">Error loading panorama: {pageState.error.description}</p>
		{:else if pageState.isLoading}
			<p aria-busy="true">Loading panorama...</p>
		{:else if pageState.panorama}
			<dl>
				<dt>ID</dt>
				<dd>{pageState.panorama.id}</dd>

				<dt>Created At</dt>
				<dd>{formatDateTimeISO(pageState.panorama.created_at)}</dd>

				<dt>Title</dt>
				<dd>{pageState.panorama.title}</dd>

				<dt>Status</dt>
				<dd>{pageState.panorama.status}</dd>

				<dt>Visible</dt>
				<dd>{pageState.panorama.visible ? 'Yes' : 'No'}</dd>

				<dt>Image Count</dt>
				<dd>{pageState.panorama.image_count}</dd>

				<dt>Source Video Path</dt>
				<dd>{pageState.panorama.source_video_path ?? '(none)'}</dd>

				<dt>GPS Track Path</dt>
				<dd>{pageState.panorama.gpx_path ?? '(none)'}</dd>

				<dt>Web Video Path</dt>
				<dd>{pageState.panorama.web_video_path ?? '(none)'}</dd>

				<dt>Transcode job id</dt>
				<dd>{pageState.panorama.transcode_arn ?? '(none)'}</dd>

				<dt>Transcode job status</dt>
				<dd>{pageState.panorama.transcode_status ?? '(none)'}</dd>

				<dt>Video Timestamp</dt>
				<dd>{pageState.panorama.video_timestamp ?? 'N/A'}</dd>
			</dl>

			<Buttons>
				<Button link="/admin/panoramas/{id}/edit">Edit Panorama</Button>
				<Button link="/admin/panoramas" type="cancel">Back to List</Button>
			</Buttons>

			{#if !pageState.panorama.source_video_path}
				<VideoUploader panoramaId={id} onUploadSuccess={() => pageState.reload(id)} />
			{:else if !pageState.panorama.gpx_path}
				<TrackUploader panoramaId={id} onUploadSuccess={() => pageState.reload(id)} />
			{:else if pageState.panorama.transcode_status === 'FAILED'}
				<p>We could not process the uploaded file. Please contact the tech support.</p>
			{:else if pageState.panorama.transcode_status !== 'SUCCEEDED'}
				<p>We are processing the uploaded video file, please wait.</p>
				<p>You will get an email when we're ready for next steps.</p>
			{:else}
				<VideoPlayer panoramaId={id} bind:offset={videoOffset} />
				<p>Video frame offset: {videoOffset}</p>
				<TrackPreview panoramaId={id} />
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
		gap: 2rem;
	}

	dl {
		display: grid;
		grid-template-columns: max-content 1fr;
		gap: 0.5rem 1rem;
	}

	dt {
		font-weight: bold;
	}

	dd {
		margin: 0;
	}
</style>
