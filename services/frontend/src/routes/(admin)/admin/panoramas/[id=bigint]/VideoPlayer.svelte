<script lang="ts">
	import { componentState } from './VideoPlayer.svelte.ts';

	const { panoramaId }: { panoramaId: string } = $props();

	$effect(() => {
		componentState.load(panoramaId);
	});
</script>

<section>
	<h3>Processed Video</h3>
	{#if componentState.error}
		<p class="error">{componentState.error.description}</p>
	{:else if componentState.isLoading}
		<p aria-busy="true">Loading video...</p>
	{:else if componentState.videoUrl}
		<!-- svelte-ignore a11y_media_has_caption -->
		<video src={componentState.videoUrl} controls preload="metadata"></video>
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

	video {
		width: 100%;
		max-height: 500px;
		border-radius: var(--pico-border-radius);
		background-color: var(--pico-background-color);
	}
</style>
