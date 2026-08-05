<script lang="ts">
	import { VideoUploaderState } from './VideoUploader.svelte.ts';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import { formatSize } from '$lib/utils/strings';

	const {
		panoramaId,
		onUploadSuccess
	}: {
		panoramaId: string;
		onUploadSuccess: () => void;
	} = $props();

	const componentState = new VideoUploaderState();

	let fileInput = $state<HTMLInputElement>();
	let selectedFile = $state<File | undefined>();

	const handleUploadClick = () => {
		fileInput?.click();
	};

	const handleFileChange = async (event: Event) => {
		const target = event.target as HTMLInputElement;
		const file = target.files?.[0];
		if (file) {
			selectedFile = file;
			await componentState.uploadVideo(panoramaId, file, onUploadSuccess);
		}
	};

	const handleRetry = async () => {
		if (selectedFile) {
			await componentState.uploadVideo(panoramaId, selectedFile, onUploadSuccess);
		}
	};

	const handleStartOver = () => {
		selectedFile = undefined;
		componentState.reset();
	};
</script>

<section>
	<h3>Video file</h3>
	<p>The video file is missing and needs to be uploaded to start processing.</p>

	{#if componentState.error}
		<p class="error">{componentState.error.description}</p>
	{/if}

	{#if componentState.isUploading}
		<div class="upload-progress">
			<div class="upload-status">
				<span>Uploading video: {componentState.uploadProgress}%</span>
				<span
					>{formatSize(componentState.uploadedBytes)} / {formatSize(
						componentState.totalBytes
					)}</span
				>
			</div>
			<progress value={componentState.uploadProgress} max="100"></progress>
		</div>
	{:else if componentState.error}
		<Buttons>
			<Button onClick={handleRetry}>Retry Upload</Button>
			<Button onClick={handleStartOver} type="secondary">Start Over</Button>
		</Buttons>
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
		margin-top: var(--gap);
		padding-top: var(--gap);
		border-top: 1px solid var(--sep-color);
	}

	.error {
		color: var(--color-learn-wrong-bg);
	}

	.upload-progress {
		margin-top: var(--gap);
		width: 100%;
	}

	.upload-status {
		display: flex;
		justify-content: space-between;
		margin-bottom: calc(var(--gap) * 0.5);
		font-size: 0.9rem;
		color: var(--text-color-inactive);
	}

	progress {
		display: block;
		width: 100%;
		height: 16px;
		border-radius: 8px;
		background-color: var(--color-progress-bg);
		border: none;
		appearance: none;
		-webkit-appearance: none;
	}

	progress::-webkit-progress-bar {
		background-color: var(--color-progress-bg);
		border-radius: 8px;
	}

	progress::-webkit-progress-value {
		background-color: var(--color-progress-fg);
		border-radius: 8px;
		transition: width 0.3s ease;
	}

	progress::-moz-progress-bar {
		background-color: var(--color-progress-fg);
		border-radius: 8px;
	}
</style>
