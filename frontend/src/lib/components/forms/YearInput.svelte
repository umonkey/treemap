<script lang="ts">
	import NumberInput from '$lib/components/forms/NumberInput.svelte';
	import { locale } from '$lib/locale';

	const {
		value,
		onChange
	}: {
		value: number | null;
		onChange: (value: number) => void;
	} = $props();
</script>

<NumberInput
	label={locale.yearLabel()}
	{value}
	{onChange}
	help="https://github.com/KanachYerevan/kb/wiki/Year-Planted"
/>
