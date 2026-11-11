<script lang="ts">
	import { getContext } from 'svelte';
	import { page } from '$app/state';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import VideoUploader from './VideoUploader.svelte';
	import TrackUploader from './TrackUploader.svelte';
	import PanoramaPreview from './PanoramaPreview.svelte';
	import PanoramaInfo from './PanoramaInfo.svelte';
	import { PANORAMA_CONTEXT_KEY, type PanoramaLayoutState } from './layout.svelte.ts';
	import { PageState } from './page.svelte.ts';

	const id = $derived(page.params.id as string);
	const state = getContext<PanoramaLayoutState>(PANORAMA_CONTEXT_KEY);
	const pageState = new PageState();
</script>

<article>
	{#if state.isLoading}
		<p aria-busy="true">Loading panorama...</p>
	{:else if state.panorama}
		<PanoramaInfo panorama={state.panorama} />

		{#if state.panorama.status === 'NEEDS_FILES'}
			{#if !state.panorama.gpx_path}
				<TrackUploader panoramaId={id} onUploadSuccess={() => state.reload(id)} />
			{/if}

			{#if !state.panorama.source_video_path}
				<VideoUploader panoramaId={id} onUploadSuccess={() => state.reload(id)} />
			{/if}
		{:else if state.panorama.status === 'FAILURE'}
			<p class="error">
				We could not process the uploaded file: {state.panorama.failure_reason ||
					'Please contact technical support.'}
			</p>
		{:else if state.panorama.status === 'NEEDS_PROCESSING' || state.panorama.status === 'NEEDS_PROCESSING_FINISH'}
			<p>
				We are now processing your dataset: extracting images, correcting their position, etc. When
				we finish, you will get an email.
			</p>
		{:else if state.panorama.status === 'SUCCESS'}
			<PanoramaPreview panoramaId={id} />
		{/if}

		{#if state.panorama.status === 'SUCCESS' || state.panorama.status === 'FAILURE'}
			<Buttons>
				<Button
					type="secondary"
					disabled={pageState.isExporting}
					onClick={() => pageState.exportData(id)}>Download data</Button
				>
			</Buttons>
		{/if}
	{/if}
</article>

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
</style>
