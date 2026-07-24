<script lang="ts">
	import { untrack } from 'svelte';
	import { componentState } from './VideoSync.svelte.ts';
	import { updatePanorama } from '$lib/api/panoramas';
	import { pageState } from './page.svelte.ts';
	import VideoPlayer from './VideoPlayer.svelte';
	import TrackPreview from './TrackPreview.svelte';
	import NumberInput from '$lib/ui/number-input/NumberInput.svelte';
	import Button from '$lib/ui/button/Button.svelte';

	const { panoramaId }: { panoramaId: string } = $props();

	$effect(() => {
		const offset = pageState.panorama?.gpx_offset;
		if (offset != null) {
			untrack(() => {
				componentState.manualOffset = offset;
			});
		}
	});

	let isSaving = $state(false);

	const saveOffset = async () => {
		if (isSaving) return;
		isSaving = true;
		const res = await updatePanorama(panoramaId, {
			gpx_offset: componentState.manualOffset
		});
		isSaving = false;
		if (res.status === 200 && res.data) {
			pageState.panorama = res.data;
		}
	};
</script>

<div class="video-sync">
	<div class="grid">
		<div class="cell">
			<VideoPlayer {panoramaId} bind:offset={componentState.videoOffset} />
		</div>
		<div class="cell">
			<TrackPreview
				{panoramaId}
				offset={componentState.videoOffset + componentState.manualOffset}
			/>
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
		<div class="cell">
			<div class="save-button">
				<Button onClick={saveOffset} disabled={isSaving}>Save Offset</Button>
			</div>
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

	.save-button {
		margin-top: 2rem;
	}

	@media (max-width: 768px) {
		.grid {
			grid-template-columns: 1fr;
		}
	}
</style>
