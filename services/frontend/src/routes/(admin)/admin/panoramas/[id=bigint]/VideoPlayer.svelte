<script lang="ts">
	import { VideoPlayerState } from './VideoPlayer.svelte.ts';

	let { panoramaId, offset = $bindable(0) }: { panoramaId: string; offset: number } = $props();

	const playerState = new VideoPlayerState();

	let containerElement: HTMLElement | null = $state(null);

	$effect(() => {
		playerState.load(panoramaId);
	});

	$effect(() => {
		if (containerElement && playerState.videoUrl) {
			playerState.init(containerElement, playerState.videoUrl);
		}
		return () => {
			playerState.destroy();
		};
	});

	$effect(() => {
		offset = playerState.currentTime;
	});
</script>

<section>
	<h3>Processed Video (360°)</h3>
	{#if playerState.error}
		<p class="error">{playerState.error.description}</p>
	{:else if playerState.isLoading}
		<p aria-busy="true">Loading video...</p>
	{:else}
		<div bind:this={containerElement} class="viewer-container"></div>
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

	.viewer-container {
		width: 100%;
		aspect-ratio: 2/1;
		border-radius: var(--pico-border-radius);
		background-color: var(--pico-background-color);
	}
</style>
