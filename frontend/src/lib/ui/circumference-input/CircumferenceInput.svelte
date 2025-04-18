<script lang="ts">
	import { locale } from '$lib/locale';
	import { NumberInput } from '$lib/ui';

	const { value, onChange } = $props<{
		value: number | null;
		onChange: (value: number) => void;
	}>();
</script>

<NumberInput
	label={locale.circumferenceLabel()}
	value={value ? value * 100 : null}
	onChange={(value: number) => onChange(value / 100)}
	hint={locale.circumferenceHint()}
	help={locale.helpCircumference()}
/>
