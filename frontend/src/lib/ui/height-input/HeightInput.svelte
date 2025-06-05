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
	label={locale.heightLabel()}
	{value}
	{onChange}
	{autofocus}
	placeholder='0.0'
	step='0.5'
	help="https://github.com/KanachYerevan/kb/wiki/Measuring-tree-height"
/>
