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
	label={locale.heightLabel()}
	{value}
	{onChange}
	help="https://github.com/KanachYerevan/kb/wiki/Measuring-tree-height"
/>
