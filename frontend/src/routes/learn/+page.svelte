<script lang="ts">
	import ProgressBar from '$lib/components/learn/ProgressBar.svelte';
	import Header from '$lib/components/tree/Header.svelte';
	import { locale } from '$lib/locale';

	const { data } = $props();
	const { questions } = data;

	let idx = $state(0);
	let selected = $state('');
	let state = $state('guessing');

	const onSelect = (value: string) => {
		selected = value;
	};

	const onConfirm = () => {
		if (questions[idx].correct.includes(selected)) {
			state = 'correct';
		} else {
			state = 'wrong';
		}
	};

	const onContinue = () => {
		state = 'guessing';
		selected = '';
		idx += 1;

		console.debug(`[learn] idx=${idx}, selected=${selected}, state=${state}`);
	};
</script>

<svelte:head>
	<title>{locale.learnTitle()}</title>
</svelte:head>

<Header title={locale.learnTitle()} />

<div class="question padded">
	<ProgressBar total={questions.length} complete={idx} />

	<h1>{questions[idx].question}</h1>

	<img src={questions[idx].image} alt="question" />

	<div class="answers">
		{#each questions[idx].options as option}
			<button type="button" onclick={() => onSelect(option)} class:selected={option === selected}
				>{option}</button
			>
		{/each}
	</div>

	{#if state === 'guessing'}
		<div class="actions">
			<button class="button" type="button" disabled={!selected} onclick={onConfirm}>Confirm</button>
		</div>
	{:else if state === 'correct'}
		<div class="result correct">
			<div class="message">
				<p><strong>Correct!</strong></p>
			</div>
			<button type="button" class="button" onclick={onContinue}>Continue</button>
		</div>
	{:else}
		<div class="result wrong">
			<div class="message">
				<p><strong>Wrong!</strong></p>
				<p>Correct answer: {questions[idx].correct[0]}</p>
			</div>
			<button type="button" class="button" onclick={onContinue}>Continue</button>
		</div>
	{/if}
</div>

<style>
	.padded {
		margin-top: var(--gap);
	}

	img {
		width: 100%;
		aspect-ratio: 1/1;
		object-position: center;
		object-fit: cover;
	}

	.answers {
		margin: var(--gap) 0;
		display: flex;
		flex-direction: row;
		flex-wrap: wrap;
		gap: var(--gap);

		button {
			background-color: var(--form-background);
			color: var(--color);
			border: solid 1px var(--sep-color);
			padding: var(--gap);
			border-radius: 6px;
			cursor: pointer;

			&.selected {
				background-color: var(--icon-color-secondary);
			}
		}
	}

	.actions {
		margin-top: calc(var(--gap) * 3);
	}

	.result {
		.message {
			margin: var(--gap) 0;
		}
	}

	.result.correct {
		.message {
			color: #080;
			border-left: solid 8px #080;
			padding-left: var(--gap);

			p {
				margin: 0;
				line-height: 1.5;
			}
		}
	}

	.result.wrong {
		.message {
			color: #800;
			border-left: solid 8px #800;
			padding-left: var(--gap);

			p {
				margin: 0;
				line-height: 1.5;
			}
		}
	}
</style>
