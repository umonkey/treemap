<script lang="ts">
	import FormElement from '$lib/ui/form-element/FormElement.svelte';
	import Button from '$lib/ui/button/Button.svelte';

	const {
		value,
		placeholder = undefined,
		label = undefined,
		hint = undefined,
		help = undefined,
		min = 0,
		max = undefined,
		step = 1,
		autofocus,
		compact = false,
		onChange = () => {}
	} = $props<{
		value: number;
		placeholder?: string | undefined;
		label?: string | undefined;
		hint?: string | undefined;
		help?: string | undefined;
		autofocus?: boolean;
		min?: number;
		max?: number;
		step?: number;
		compact?: boolean;
		onChange: (value: number) => void;
	}>();

	const handleInput = (e: Event) => {
		const target = e.target as HTMLInputElement;
		if (target) {
			const newValue = target.valueAsNumber;
			if (!isNaN(newValue)) {
				onChange(newValue);
			}
		}
	};

	const increment = () => {
		const newValue = value + step;
		if (max === undefined || newValue <= max) {
			onChange(newValue);
		}
	};

	const decrement = () => {
		const newValue = value - step;
		if (min === undefined || newValue >= min) {
			onChange(newValue);
		}
	};
</script>

{#if label || hint || help}
	<FormElement {label} {hint} {help}>
		{@render stepper()}
	</FormElement>
{:else}
	{@render stepper()}
{/if}

{#snippet stepper()}
	<div class="stepper" class:compact>
		<Button type="secondary" onClick={decrement} disabled={min !== undefined && value <= min}>
			-
		</Button>

		<!-- svelte-ignore a11y_autofocus -->
		<input
			class="form-element"
			type="number"
			{value}
			{placeholder}
			{autofocus}
			{min}
			{max}
			{step}
			inputmode="numeric"
			oninput={handleInput}
		/>

		<Button type="secondary" onClick={increment} disabled={max !== undefined && value >= max}>
			+
		</Button>
	</div>
{/snippet}

<style>
	.stepper {
		display: flex;
		flex-direction: row;
		gap: var(--gap);
		align-items: center;

		input {
			text-align: center;
			flex-grow: 1;
			min-width: 0;

			/* Restore standard input styles when not wrapped in FormElement */
			padding: var(--gap);
			box-sizing: border-box;
			background-color: transparent;
			border: 1px solid var(--sep-color);
			border-radius: 6px;
			color: var(--form-color);
			outline: none;
			line-height: 1.25em;
		}

		&.compact input {
			flex-grow: 0;
			width: 60px;
		}

		:global(.button) {
			flex-shrink: 0;
			width: 40px;
			height: 40px;
			padding: 0;
			display: flex;
			align-items: center;
			justify-content: center;
			font-size: 1.5rem;
		}
	}
</style>
