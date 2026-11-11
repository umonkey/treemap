<script lang="ts">
	import { locale } from '$lib/locale';
	import TextInput from '$lib/ui/text-input/TextInput.svelte';

	const {
		value,
		hint,
		onChange,
		disabled = false
	}: {
		value: string;
		hint?: string;
		onChange: (value: string) => void;
		disabled?: boolean;
	} = $props();

	const handleChange = (value: string) => {
		onChange(value);
	};
</script>

<TextInput
	label={locale.commentLabel()}
	{value}
	multiline={true}
	hint={hint ?? locale.commentHint()}
	onChange={handleChange}
	{disabled}
/>
