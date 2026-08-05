<script lang="ts">
	import Button from '$lib/ui/button/Button.svelte';
	import LeftButton from '$lib/icons/LeftButton.svelte';
	import RightButton from '$lib/icons/RightButton.svelte';
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
	{#if playerState.error}
		<p class="error">{playerState.error.description}</p>
	{:else if playerState.isLoading}
		<p aria-busy="true">Loading video...</p>
	{:else}
		<div class="player-wrapper">
			<div bind:this={containerElement} class="viewer-container"></div>
			<div class="seek-button-left">
				<Button square={true} type="button" onClick={() => playerState.seek(-0.1)}>
					<LeftButton />
				</Button>
			</div>
			<div class="seek-button-right">
				<Button square={true} type="button" onClick={() => playerState.seek(0.1)}>
					<RightButton />
				</Button>
			</div>
		</div>
	{/if}
</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		height: 100%;
	}

	.error {
		color: var(--color-learn-wrong-bg, red);
	}

	.player-wrapper {
		position: relative;
		width: 100%;
		height: 100%;
	}

	.seek-button-left {
		position: absolute;
		top: 10px;
		left: 10px;
		z-index: 10;
	}

	.seek-button-right {
		position: absolute;
		top: 10px;
		right: 10px;
		z-index: 10;
	}

	.viewer-container {
		width: 100%;
		height: 100%;
		aspect-ratio: 1;
		border-radius: var(--pico-border-radius);
		background-color: var(--pico-background-color);

		:global(.psv-video-overlay) {
			display: none;
		}
	}
</style>
