<script lang="ts">
	import { locale } from '$lib/locale';
	import { NumberInput } from '$lib/ui';

	const { value, onChange } = $props<{
		value: number | null;
		onChange: (value: number) => void;
	}>();
</script>

<NumberInput
	label={locale.yearLabel()}
	{value}
	{onChange}
	help="https://github.com/KanachYerevan/kb/wiki/Year-Planted"
/>
