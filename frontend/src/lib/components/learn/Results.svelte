<script lang="ts">
	import { locale } from '$lib/locale';
	import { onMount } from 'svelte';
	import { soundBus } from '$lib/buses/soundBus';

	const { correct, total, onRetry } = $props();

	const title = (): string => {
		const rate = correct / total;

		if (rate == 1.0) {
			return locale.learnPerfect();
		}

		if (rate >= 0.8) {
			return locale.learnGreat();
		}

		if (rate >= 0.5) {
			return locale.learnGood();
		}

		return locale.learnBad();
	};

	onMount(() => {
		soundBus.emit('finished');
	});
</script>

<h1>{title()}</h1>
<p>{locale.learnScore(correct, total)}</p>
<button type="button" class="button" onclick={onRetry}>{locale.learnRetry()}</button>
