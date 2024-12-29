<script lang="ts">
	const { question, onCorrect, onWrong } = $props();

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
		} else {
			state = 'wrong';
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
				<p>Correct answer: {question.correct[0]}</p>
			</div>
			<button type="button" class="button" onclick={onContinue}>Continue</button>
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
