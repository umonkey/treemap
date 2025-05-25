<script lang="ts">
	import { FormElement } from '$lib/ui';

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
		value: string | number;
		placeholder: string;
		label: string;
		type: string;
		multiline: boolean;
		hint: string;
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
		<textarea {placeholder} onchange={handleChange}>{value}</textarea>
	{:else}
		<input {type} {value} {placeholder} onchange={handleChange} />
	{/if}
</FormElement>
