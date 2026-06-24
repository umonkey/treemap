<script lang="ts">
	import { soundBus } from '$lib/buses/soundBus';
	import { onMount } from 'svelte';

	let soundCorrect = $state<HTMLAudioElement | null>(null);
	let soundWrong = $state<HTMLAudioElement | null>(null);
	let soundFinished = $state<HTMLAudioElement | null>(null);

	onMount(() => {
		soundBus.on('correct', () => {
			if (soundCorrect) {
				soundCorrect.play();
			}
		});

		soundBus.on('wrong', () => {
			if (soundWrong) {
				soundWrong.play();
			}
		});

		soundBus.on('finished', () => {
			if (soundFinished) {
				soundFinished.play();
			}
		});

		console.debug('Sound player initialized.');
	});
</script>

<audio bind:this={soundCorrect} preload="auto">
	<source src="/sounds/correct.aac" type="audio/aac" />
	<source src="/sounds/correct.oga" type="audio/ogg" />
</audio>

<audio bind:this={soundWrong} preload="auto">
	<source src="/sounds/wrong.aac" type="audio/aac" />
	<source src="/sounds/wrong.oga" type="audio/ogg" />
</audio>

<audio bind:this={soundFinished} preload="auto">
	<source src="/sounds/finished.aac" type="audio/aac" />
	<source src="/sounds/finished.oga" type="audio/ogg" />
</audio>
