<script lang="ts">
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import ProgressBar from './ProgressBar.svelte';
	import Question from './Question.svelte';
	import Results from './Results.svelte';
	import SoundPlayer from './SoundPlayer.svelte';

	import { getRandomQuestions } from '$lib/learn/questions';
	import { locale } from '$lib/locale';

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

<Dialog title={locale.learnTitle()}>
	<ProgressBar total={questions.length} complete={idx} />

	{#if idx === questions.length}
		<Results {correct} total={questions.length} {onRetry} />
	{:else}
		<Question question={questions[idx]} {onCorrect} {onWrong} />
	{/if}

	<SoundPlayer />
</Dialog>
