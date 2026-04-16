<script lang="ts">
	import { locale } from '$lib/locale';
	import NumberInput from '$lib/ui/number-input/NumberInput.svelte';
	import { componentState as state } from './RowSizeInput.svelte.ts';

	const { value, distance, autofocus, onChange } = $props<{
		value: number;
		distance: number;
		autofocus?: boolean;
		onChange: (value: number) => void;
	}>();

	$effect(() => {
		state.distance = distance;
		state.value = value;
		state.onChange = onChange;
	});
</script>

<NumberInput
	label={locale.rowSizeLabel()}
	hint={state.hint}
	{value}
	onChange={state.handleChange}
	{autofocus}
	placeholder="2"
	min="2"
	step={1}
/>
