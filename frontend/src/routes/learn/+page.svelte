<script lang="ts">
	import ProgressBar from '$lib/components/learn/ProgressBar.svelte';
	import Header from '$lib/components/tree/Header.svelte';
	import Question from '$lib/components/learn/Question.svelte';
	import Results from '$lib/components/learn/Results.svelte';
	import { locale } from '$lib/locale';

	const { data } = $props();
	const { questions } = data;

	let soundFinished;

	let idx = $state(0);
	let correct = $state(0);

	const onNext = () => {
		idx++;

		if (idx === questions.length) {
			soundFinished.play();
		}
	};

	const onCorrect = () => {
		correct++;
		onNext();
	};

	const onWrong = () => {
		onNext();
	};
</script>

<svelte:head>
	<title>{locale.learnTitle()}</title>
</svelte:head>

<Header title={locale.learnTitle()} />

<div class="question padded">
	<ProgressBar total={questions.length} complete={idx} />

	{#if idx === questions.length}
		<Results {correct} total={questions.length} />
	{:else}
		<Question question={questions[idx]} {onCorrect} {onWrong} />
	{/if}
</div>

<audio bind:this={soundFinished}>
	<source src="/sounds/finished.aac" type="audio/aac" />
	<source src="/sounds/finished.oga" type="audio/ogg" />
</audio>

<style>
	.padded {
		margin-top: var(--gap);
	}
</style>
