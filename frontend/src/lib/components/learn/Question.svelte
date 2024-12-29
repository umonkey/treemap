<script lang="ts">
	import { locale } from '$lib/locale';

	const { question, onCorrect, onWrong } = $props();

	let soundCorrect, soundWrong;

	// Which answer is selected (not yet submitted).
	let selected = $state('');

	// Values: guessing, correct, wrong.
	let state = $state('guessing');

	const onSelect = (value: string) => {
		selected = value;
	};

	const onConfirm = () => {
		if (question.correct.includes(selected)) {
			state = 'correct';
			soundCorrect.play();
		} else {
			state = 'wrong';
			soundWrong.play();
		}
	};

	// Go to the next question.
	const onContinue = () => {
		if (state === 'correct') {
			onCorrect();
		} else {
			onWrong();
		}

		state = 'guessing';
		selected = '';
	};
</script>

<div class="question">
	<h1>{question.question}</h1>

	<img src={question.image} alt="question" />

	<div class="answers">
		{#each question.options as option}
			<button type="button" onclick={() => onSelect(option)} class:selected={option === selected}
				>{option}</button
			>
		{/each}
	</div>

	{#if state === 'guessing'}
		<div class="actions">
			<button class="button" type="button" disabled={!selected} onclick={onConfirm}
				>{locale.learnConfirm()}</button
			>
		</div>
	{:else if state === 'correct'}
		<div class="result correct">
			<div class="message">
				<p><strong>{locale.learnCorrect()}</strong></p>
			</div>
			<button type="button" class="button" onclick={onContinue}>{locale.learnContinue()}</button>
		</div>
	{:else}
		<div class="result wrong">
			<div class="message">
				<p><strong>{locale.learnWrong()}</strong></p>
				<p>{locale.learnCorrectAnswer(question.correct[0])}</p>
			</div>
			<button type="button" class="button" onclick={onContinue}>{locale.learnContinue()}</button>
		</div>
	{/if}

	<audio bind:this={soundCorrect}>
		<source src="/sounds/correct.aac" type="audio/aac" />
		<source src="/sounds/correct.oga" type="audio/ogg" />
	</audio>

	<audio bind:this={soundWrong}>
		<source src="/sounds/wrong.aac" type="audio/aac" />
		<source src="/sounds/wrong.oga" type="audio/ogg" />
	</audio>
</div>

<style>
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
		background-color: var(--form-background);
		width: 100%;
		padding: calc(var(--gap) * 2);
		box-sizing: border-box;
		z-index: var(--z-learn-result);

		.message {
			margin: var(--gap) 0;
		}
	}

	.result.correct {
		.message {
			color: var(--color-learn-correct-bg);

			p {
				margin: 0;
				line-height: 1.5;
			}
		}

		button {
			background-color: var(--color-learn-correct-bg);
			border-color: var(--color-learn-correct-bg);
			color: var(--color-learn-correct-fg);
		}
	}

	.result.wrong {
		.message {
			color: var(--color-learn-wrong-bg);

			p {
				margin: 0;
				line-height: 1.5;
			}
		}

		button {
			background-color: var(--color-learn-wrong-bg);
			border-color: var(--color-learn-wrong-bg);
			color: var(--color-learn-wrong-fg);
		}
	}

	@media (max-width: 480px) {
		.result {
			position: fixed;
			bottom: 0;
			left: 0;
			border-top-left-radius: 8px;
			border-top-right-radius: 8px;
			font-size: 20px;

			button {
				width: 100%;
				font-size: 18px;
			}
		}
	}
</style>
