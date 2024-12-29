<script lang="ts">
	import ProgressBar from '$lib/components/learn/ProgressBar.svelte';
	import Header from '$lib/components/tree/Header.svelte';
	import Question from '$lib/components/learn/Question.svelte';
	import Results from '$lib/components/learn/Results.svelte';
	import SoundPlayer from '$lib/components/learn/SoundPlayer.svelte';

	import { locale } from '$lib/locale';
	import { getRandomQuestions } from '$lib/learn/questions';

	let questions = $state(getRandomQuestions());
	let idx = $state(0);
	let correct = $state(0);

	const onCorrect = () => {
		correct++;
		idx++;
	};

	const onWrong = () => {
		idx++;
	};

	const onRetry = () => {
		idx = 0;
		correct = 0;
		questions = getRandomQuestions();
	};
</script>

<svelte:head>
	<title>{locale.learnTitle()}</title>
</svelte:head>

<Header title={locale.learnTitle()} />

<div class="question padded">
	<ProgressBar total={questions.length} complete={idx} />

	{#if idx === questions.length}
		<Results {correct} total={questions.length} {onRetry} />
	{:else}
		<Question question={questions[idx]} {onCorrect} {onWrong} />
	{/if}
</div>

<SoundPlayer />

<style>
	.padded {
		margin-top: var(--gap);
	}
</style>
