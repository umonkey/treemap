<script lang="ts">
	import { locale } from '$lib/locale';
	import NumberInput from '$lib/components/forms/NumberInput.svelte';

	let {
		value,
		onChange
	}: {
		value: number | null;
		onChange: (value: number) => void;
	} = $props();
</script>

<NumberInput
	label={locale.circumferenceLabel()}
	value={value ? value * 100 : null}
	onChange={(value: number) => onChange(value / 100)}
	hint={locale.circumferenceHint()}
	help={locale.helpCircumference()}
/>
