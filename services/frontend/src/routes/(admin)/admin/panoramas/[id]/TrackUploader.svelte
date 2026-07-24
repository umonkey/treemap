<script lang="ts">
	import { componentState } from './TrackUploader.svelte.ts';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';

	const {
		panoramaId,
		onUploadSuccess
	}: {
		panoramaId: string;
		onUploadSuccess: () => void;
	} = $props();

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
			await componentState.uploadTrack(panoramaId, file, onUploadSuccess);
		}
	};

	const handleRetry = async () => {
		if (selectedFile) {
			await componentState.uploadTrack(panoramaId, selectedFile, onUploadSuccess);
		}
	};

	const handleStartOver = () => {
		selectedFile = undefined;
		componentState.reset();
	};
</script>

<section>
	<h3>GPS track</h3>
	<p>The GPS track file is missing and needs to be uploaded.</p>

	{#if componentState.error}
		<p class="error">{componentState.error.description}</p>
	{/if}

	{#if componentState.isUploading}
		<div class="upload-progress">
			<div class="upload-status">
				<span>Uploading GPS track...</span>
			</div>
			<progress></progress>
		</div>
	{:else if componentState.error}
		<Buttons>
			<Button onClick={handleRetry}>Retry Upload</Button>
			<Button onClick={handleStartOver} type="secondary">Start Over</Button>
		</Buttons>
	{:else}
		<input
			type="file"
			accept=".gpx,application/gpx+xml"
			bind:this={fileInput}
			onchange={handleFileChange}
			style="display: none;"
		/>
		<Button onClick={handleUploadClick}>Upload GPS track</Button>
	{/if}
</section>

<style>
	section {
		margin-top: var(--gap);
		padding-top: var(--gap);
		border-top: 1px solid var(--sep-color);
	}

	.error {
		color: var(--color-learn-wrong-bg, red);
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
		background-color: var(--color-progress-bg, #eee);
		border: none;
		appearance: none;
		-webkit-appearance: none;
	}

	progress::-webkit-progress-bar {
		background-color: var(--color-progress-bg, #eee);
		border-radius: 8px;
	}

	progress::-webkit-progress-value {
		background-color: var(--color-progress-fg, #333);
		border-radius: 8px;
	}

	progress::-moz-progress-bar {
		background-color: var(--color-progress-fg, #333);
		border-radius: 8px;
	}
</style>
