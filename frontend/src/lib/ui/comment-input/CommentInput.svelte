<script lang="ts">
	import { locale } from '$lib/locale';
	import TextInput from '../text-input/TextInput.svelte';

	const { value, hint, onChange } = $props<{
		value: string;
		hint?: string;
		onChange: (value: string) => void;
	}>();

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
/>
