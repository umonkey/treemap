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
		onChange
	} = $props<{
		value?: string | number | null;
		placeholder?: string;
		label?: string;
		type?: string;
		multiline?: boolean;
		hint?: string;
		help?: string;
		onChange: (value: string) => void;
	}>();

	const handleChange = (e: Event) => {
		if (e.target) {
			const em = e.target as HTMLInputElement;
			onChange(em.value ?? '');
		}
	};
</script>

<FormElement {label} {hint} {help}>
	{#if multiline}
		<textarea {placeholder} oninput={handleChange}>{value}</textarea>
	{:else}
		<input {type} {value} {placeholder} oninput={handleChange} />
	{/if}
</FormElement>
