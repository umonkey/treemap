<script lang="ts">
	import { apiClient } from '$lib/api';
	import { soundBus } from '$lib/buses/soundBus';
	import { locale } from '$lib/locale';
	import { onMount } from 'svelte';

	const { correct, total, onRetry } = $props<{
		correct: number;
		total: number;
		onRetry: () => void;
	}>();

	const title = (): string => {
		const rate = correct / total;

		if (rate === 1.0) {
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

		const rate = correct / total;

		apiClient.addTraining(rate);
	});
</script>

<h1>{title()}</h1>
<p>{locale.learnScore(correct, total)}</p>
<button type="button" class="button" onclick={onRetry}>{locale.learnRetry()}</button>
