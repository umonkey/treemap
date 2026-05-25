<script lang="ts">
	import { locale } from '$lib/locale';
	import StepperInput from '$lib/ui/stepper-input/StepperInput.svelte';

	const { value, distance, autofocus, onChange } = $props<{
		value: number;
		distance: number;
		autofocus?: boolean;
		onChange: (value: number) => void;
	}>();

	const step = $derived(
		distance > 0 && value > 1 ? Math.round((distance / (value - 1)) * 10) / 10 : 0
	);
	const hint = $derived(locale.rowStepInfo(value, step, distance));
</script>

<div class="row-size-input">
	<div class="label-row">
		<span class="label">{locale.rowSizeLabel()}</span>
		<div class="input-container">
			<StepperInput {value} {onChange} {autofocus} placeholder="2" min={2} step={1} compact />
		</div>
	</div>
	{#if hint}
		<div class="hint">{hint}</div>
	{/if}
</div>

<style>
	.row-size-input {
		display: flex;
		flex-direction: column;
		gap: var(--gap);

		.label-row {
			display: flex;
			flex-direction: column;
			gap: var(--gap);

			.label {
				opacity: 0.75;
			}

			.input-container {
				width: 100%;
			}
		}

		.hint {
			color: var(--text-color-inactive);
			font-size: 0.85em;
			line-height: 125%;
		}
	}

	@media screen and (max-width: 1023px) {
		.row-size-input {
			.label-row {
				flex-direction: row;
				justify-content: space-between;
				align-items: center;

				.input-container {
					width: auto;
				}
			}
		}
	}
</style>
