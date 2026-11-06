<script lang="ts">
	import { untrack } from 'svelte';
	import { locale } from '$lib/locale';
	import NumberInput from '$lib/ui/number-input/NumberInput.svelte';
	import SelectButton from '$lib/ui/SelectButton.svelte';

	const {
		value,
		autofocus,
		showDivider = false,
		onChange
	}: {
		value: number | null;
		autofocus?: boolean;
		showDivider?: boolean;
		onChange: (value: number) => void;
	} = $props();

	let currentValue = $state(untrack(() => value));

	$effect(() => {
		const incoming = value;

		untrack(() => {
			if (incoming !== currentValue) currentValue = incoming;
		});
	});

	const suggestion = $derived(
		showDivider && currentValue !== null && currentValue > 0 ? (currentValue / 2).toFixed(1) : null
	);

	const handleChange = (v: number) => {
		currentValue = v;
		onChange(v);
	};

	const handleSuggestionClick = (v: string) => {
		const parsed = Number(v);
		if (Number.isFinite(parsed)) {
			currentValue = parsed;
			onChange(parsed);
		}
	};
</script>

<NumberInput
	label={locale.canopyLabel()}
	help="https://github.com/KanachYerevan/kb/wiki/Measuring-tree-crown"
	placeholder="0.0"
	{autofocus}
	value={currentValue}
	onChange={handleChange}
>
	{#if suggestion && parseFloat(suggestion) > 0}
		<div class="suggested">
			<SelectButton value={suggestion} label={suggestion} onClick={handleSuggestionClick} />
		</div>
	{/if}
</NumberInput>

<style>
	.suggested {
		padding: 0;
		margin: 1rem 0 0;

		display: flex;
		flex-direction: row;
		flex-wrap: wrap;
		gap: 0.5rem;
	}
</style>
