<script lang="ts">
	import { locale } from '$lib/locale';
	import FormElement from '$lib/ui/form-element/FormElement.svelte';
	import SelectButton from '$lib/ui/SelectButton.svelte';

	const {
		value,
		autofocus,
		onChange
	}: {
		value: number | null;
		autofocus?: boolean;
		onChange: (value: number) => void;
	} = $props();

	let currentValue = $state<number | null>(null);

	$effect(() => {
		currentValue = value;
	});

	const handleInput = (e: Event) => {
		const target = e.target as HTMLInputElement;
		currentValue = isNaN(target.valueAsNumber) ? null : target.valueAsNumber;
		if (currentValue !== null) {
			onChange(currentValue);
		}
	};

	const suggestion = $derived(
		currentValue !== null && currentValue > 0 ? (currentValue / 2).toFixed(1) : null
	);

	const handleSuggestionClick = (v: string) => {
		const num = parseFloat(v);
		currentValue = num;
		onChange(num);
	};
</script>

<FormElement
	label={locale.canopyLabel()}
	help="https://github.com/KanachYerevan/kb/wiki/Measuring-tree-crown"
>
	<!-- svelte-ignore a11y_autofocus -->
	<input
		type="number"
		value={currentValue}
		{autofocus}
		placeholder="0.0"
		step="0.1"
		min="0"
		oninput={handleInput}
	/>

	{#if suggestion && parseFloat(suggestion) > 0}
		<div class="suggested">
			<SelectButton value={suggestion} label={suggestion} onClick={handleSuggestionClick} />
		</div>
	{/if}
</FormElement>

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
