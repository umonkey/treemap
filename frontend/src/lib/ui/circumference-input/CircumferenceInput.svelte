<script lang="ts">
	import { locale } from '$lib/locale';
	import { NumberInput } from '$lib/ui';

	const { value, autofocus, onChange } = $props<{
		value: number | null;
		autofocus?: boolean;
		onChange: (value: number) => void;
	}>();
</script>

<NumberInput
	label={locale.circumferenceLabel()}
	value={value ? value * 100 : null}
	{autofocus}
	placeholder="0.0"
	step="0.5"
	onChange={(value: number) => onChange(value / 100)}
	hint={locale.circumferenceHint()}
	help={locale.helpCircumference()}
/>
