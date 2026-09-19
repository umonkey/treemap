<script lang="ts">
	import TextInput from '$lib/ui/text-input/TextInput.svelte';
	import { locale } from './DisplayNameInput.lang';

	let {
		value = $bindable(''),
		onSave
	}: {
		value?: string;
		onSave?: (name: string) => void;
	} = $props();

	const handleChange = (next: string) => {
		value = next;
	};

	const handleBlur = (next: string) => {
		onSave?.(next);
	};
</script>

<TextInput label={locale.displayNameLabel()} {value} onChange={handleChange} onBlur={handleBlur} />
