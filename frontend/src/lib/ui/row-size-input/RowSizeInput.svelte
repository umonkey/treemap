<script lang="ts">
	import { locale } from '$lib/locale';
	import { NumberInput } from '$lib/ui';
	import { hooks } from './hooks';

	const { value, distance, autofocus, onChange } = $props<{
		value: number | null;
		distance: number;
		autofocus?: boolean;
		onChange: (value: number) => void;
	}>();

	const { hint, handleChange } = hooks({ distance, value, onChange });
</script>

<NumberInput
	label={locale.rowSizeLabel()}
	hint={$hint}
	{value}
	onChange={handleChange}
	{autofocus}
	placeholder="2"
	min="2"
	step="1"
/>
