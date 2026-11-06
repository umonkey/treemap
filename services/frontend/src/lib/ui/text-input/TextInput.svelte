<script lang="ts">
	import FormElement from '$lib/ui/form-element/FormElement.svelte';

	const {
		value = '',
		placeholder = '',
		label = '',
		type = 'text',
		multiline = false,
		hint = '',
		help,
		onChange,
		onBlur,
		ariaLabel = ''
	}: {
		value?: string | number | null;
		placeholder?: string;
		label?: string;
		type?: string;
		multiline?: boolean;
		hint?: string;
		help?: string;
		onChange: (value: string) => void;
		onBlur?: (value: string) => void;
		ariaLabel?: string;
	} = $props();

	const handleChange = (e: Event) => {
		if (e.target) {
			const em = e.target as HTMLInputElement;
			onChange(em.value ?? '');
		}
	};

	const handleBlur = (e: Event) => {
		if (e.target) {
			const em = e.target as HTMLInputElement;
			onBlur?.(em.value ?? '');
		}
	};
</script>

<FormElement {label} {hint} {help}>
	{#if multiline}
		<textarea
			{placeholder}
			aria-label={ariaLabel || undefined}
			oninput={handleChange}
			onblur={handleBlur}>{value}</textarea
		>
	{:else}
		<input
			{type}
			{value}
			{placeholder}
			aria-label={ariaLabel || undefined}
			oninput={handleChange}
			onblur={handleBlur}
		/>
	{/if}
</FormElement>
