<script lang="ts">
import NumberInput from "$lib/components/forms/NumberInput.svelte";
import { locale } from "$lib/locale";

const {
	value,
	onChange,
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
