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
	label={locale.canopyLabel()}
	{value}
	{onChange}
	help="https://github.com/KanachYerevan/kb/wiki/Measuring-tree-crown"
/>
