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
		onChange,
		onKeyDown
	} = $props<{
		value: string | number;
		placeholder: string;
		label: string;
		type: string;
		multiline: boolean;
		hint: string;
		help?: string;
		onChange: (value: string) => void;
		onKeyDown?: (event: KeyboardEvent) => void;
	}>();

	const handleChange = (e: Event) => {
		if (e.target) {
			const em = e.target as HTMLInputElement;
			onChange(em.value ?? '');
		}
	};

	const handleKeyDown = (e: KeyboardEvent) => {
		if (onKeyDown) {
			onKeyDown(e);
		}
	};
</script>

<FormElement {label} {hint} {help}>
	{#if multiline}
		<textarea {placeholder} {value} onchange={handleChange} onkeydown={handleKeyDown}></textarea>
	{:else}
		<input {type} {value} {placeholder} onchange={handleChange} onkeydown={handleKeyDown} />
	{/if}
</FormElement>
