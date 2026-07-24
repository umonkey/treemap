<script lang="ts">
	import { componentState } from './VideoPlayer.svelte.ts';

	const { panoramaId }: { panoramaId: string } = $props();

	let containerElement: HTMLElement | null = $state(null);

	$effect(() => {
		componentState.load(panoramaId);
	});

	$effect(() => {
		if (containerElement && componentState.videoUrl) {
			componentState.init(containerElement, componentState.videoUrl);
		}
		return () => {
			componentState.destroy();
		};
	});
</script>

<section>
	<h3>Processed Video (360°)</h3>
	{#if componentState.error}
		<p class="error">{componentState.error.description}</p>
	{:else if componentState.isLoading}
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
