<script lang="ts">
	import { soundBus } from '$lib/buses/soundBus';
	import { locale } from '$lib/locale';

	const { question, onCorrect, onWrong } = $props<{
		question: {
			question: string;
			image: string;
			options: string[];
			correct: string[];
		};
		onCorrect: () => void;
		onWrong: () => void;
	}>();

	// Which answer is selected (not yet submitted).
	let selected = $state('');

	// Values: guessing, correct, wrong.
	let currentState = $state('guessing');

	const onSelect = (value: string) => {
		selected = value;
	};

	const onConfirm = () => {
		if (question.correct.includes(selected)) {
			currentState = 'correct';
			soundBus.emit('correct');
		} else {
			currentState = 'wrong';
			soundBus.emit('wrong');
		}
	};

	// Go to the next question.
	const onContinue = () => {
		if (currentState === 'correct') {
			onCorrect();
		} else {
			onWrong();
		}

		currentState = 'guessing';
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

	{#if currentState === 'guessing'}
		<div class="actions">
			<button class="button" type="button" disabled={!selected} onclick={onConfirm}
				>{locale.learnConfirm()}</button
			>
		</div>
	{:else if currentState === 'correct'}
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
