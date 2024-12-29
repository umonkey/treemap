<script lang="ts">
	import ProgressBar from '$lib/components/learn/ProgressBar.svelte';
	import Header from '$lib/components/tree/Header.svelte';
	import Question from '$lib/components/learn/Question.svelte';
	import { locale } from '$lib/locale';

	const { data } = $props();
	const { questions } = data;

	let idx = $state(0);
	let correct = $state(0);

	const onReload = () => {
		window.location.reload();
	};

	const onCorrect = () => {
		correct++;
		idx++;
	};

	const onWrong = () => {
		idx++;
	};
</script>

<svelte:head>
	<title>{locale.learnTitle()}</title>
</svelte:head>

<Header title={locale.learnTitle()} />

<div class="question padded">
	<ProgressBar total={questions.length} complete={idx} />

	{#if idx === questions.length}
		<h1>Done!</h1>
		<p>Your score is: {correct}/{questions.length}</p>
		<button type="button" class="button" onclick={onReload}>Try again</button>
	{:else}
		<Question question={questions[idx]} {onCorrect} {onWrong} />
	{/if}
</div>

<style>
	.padded {
		margin-top: var(--gap);
	}
</style>
