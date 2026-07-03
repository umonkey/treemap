<script lang="ts">
	import { componentState } from './VideoUploader.svelte.ts';
	import Button from '$lib/ui/button/Button.svelte';
	import { formatSize } from '$lib/utils/strings';

	const {
		panoramaId,
		onUploadSuccess
	}: {
		panoramaId: string;
		onUploadSuccess: () => void;
	} = $props();

	let fileInput = $state<HTMLInputElement>();

	const handleUploadClick = () => {
		fileInput?.click();
	};

	const handleFileChange = async (event: Event) => {
		const target = event.target as HTMLInputElement;
		const file = target.files?.[0];
		if (file) {
			await componentState.uploadVideo(panoramaId, file, onUploadSuccess);
		}
	};
</script>

<section>
	<h3>Video file</h3>
	<p>The video file is missing and needs to be uploaded to start processing.</p>

	{#if componentState.error}
		<p class="error">{componentState.error.description}</p>
	{/if}

	{#if componentState.isUploading}
		<p>
			Uploading video: {componentState.uploadProgress}% ({formatSize(componentState.uploadedBytes)}
			/ {formatSize(componentState.totalBytes)})
		</p>
		<progress value={componentState.uploadProgress} max="100"></progress>
	{:else}
		<input
			type="file"
			accept="video/mp4"
			bind:this={fileInput}
			onchange={handleFileChange}
			style="display: none;"
		/>
		<Button onClick={handleUploadClick}>Upload Video</Button>
	{/if}
</section>

<style>
	section {
		margin-top: var(--pico-spacing);
		padding-top: var(--pico-spacing);
		border-top: 1px solid var(--pico-muted-border-color);
	}

	.error {
		color: var(--pico-danger-color);
	}
</style>
