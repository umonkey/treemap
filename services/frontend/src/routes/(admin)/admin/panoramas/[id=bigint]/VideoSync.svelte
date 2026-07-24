<script lang="ts">
	import { componentState } from './VideoSync.svelte.ts';
	import VideoPlayer from './VideoPlayer.svelte';
	import TrackPreview from './TrackPreview.svelte';
	import NumberInput from '$lib/ui/number-input/NumberInput.svelte';

	const { panoramaId }: { panoramaId: string } = $props();
</script>

<div class="video-sync">
	<div class="grid">
		<div class="cell">
			<VideoPlayer {panoramaId} bind:offset={componentState.videoOffset} />
		</div>
		<div class="cell">
			<TrackPreview {panoramaId} offset={componentState.videoOffset + componentState.manualOffset} />
		</div>
		<div class="cell">
			<NumberInput
				label="GPS Track Offset"
				value={componentState.manualOffset}
				step="0.01"
				min={-Infinity}
				onChange={(val) => (componentState.manualOffset = val)}
			/>
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
