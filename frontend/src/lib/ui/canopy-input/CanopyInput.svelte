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
	label={locale.canopyLabel()}
	{value}
	{autofocus}
	{onChange}
	help="https://github.com/KanachYerevan/kb/wiki/Measuring-tree-crown"
/>
