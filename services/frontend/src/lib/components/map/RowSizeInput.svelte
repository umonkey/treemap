<script lang="ts">
	import { locale } from '$lib/locale';
	import NumberInput from '$lib/ui/number-input/NumberInput.svelte';

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

<NumberInput
	label={locale.rowSizeLabel()}
	{hint}
	{value}
	{onChange}
	{autofocus}
	placeholder="2"
	min="2"
	step={1}
/>
