<script lang="ts">
	import { locale } from '$lib/locale';
	import SelectButton from '$lib/ui/SelectButton.svelte';
	import FormElement from '$lib/ui/form-element/FormElement.svelte';

	const { value, onChange } = $props<{
		value: number;
		onChange: (value: string) => void;
	}>();

	const options = [
		{ id: '31536000', label: locale.ageYear() },
		{ id: '2592000', label: locale.ageMonth() },
		{ id: '604800', label: locale.ageWeek() },
		{ id: '172800', label: locale.age2Days() },
		{ id: '86400', label: locale.ageDay() }
	];
</script>

<FormElement label={locale.ageLabel()} hint={locale.ageHint()}>
	<div class="buttons">
		{#each options as option}
			<SelectButton
				value={option.id}
				label={option.label}
				active={value.toString() === option.id}
				onClick={onChange}
			/>
		{/each}
	</div>
</FormElement>

<style>
	.buttons {
		display: flex;
		flex-wrap: wrap;
		gap: 0.5rem;
	}
</style>
