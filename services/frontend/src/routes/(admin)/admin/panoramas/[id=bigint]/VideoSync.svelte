<script lang="ts">
	import { componentState } from './VideoSync.svelte.ts';
	import VideoPlayer from './VideoPlayer.svelte';
	import TrackPreview from './TrackPreview.svelte';

	const { panoramaId }: { panoramaId: string } = $props();
</script>

<div class="video-sync">
	<div class="grid">
		<div class="cell">
			<VideoPlayer {panoramaId} bind:offset={componentState.videoOffset} />
		</div>
		<div class="cell">
			<TrackPreview {panoramaId} />
		</div>
		<div class="cell">
			<p>Video frame offset: {componentState.videoOffset}</p>
		</div>
	</div>
</div>

<style>
	.video-sync {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.grid {
		display: grid;
		grid-template-columns: repeat(2, 1fr);
		gap: 1rem;
	}

	.cell {
		aspect-ratio: 1;
		width: 100%;
	}

	@media (max-width: 768px) {
		.grid {
			grid-template-columns: 1fr;
		}
	}
</style>
