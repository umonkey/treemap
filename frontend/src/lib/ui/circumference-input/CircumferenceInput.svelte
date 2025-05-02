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
	onChange={(value: number) => onChange(value / 100)}
	hint={locale.circumferenceHint()}
	help={locale.helpCircumference()}
/>
