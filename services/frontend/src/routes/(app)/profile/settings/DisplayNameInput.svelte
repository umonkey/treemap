<script lang="ts">
	import TextInput from '$lib/ui/text-input/TextInput.svelte';
	import { locale } from './DisplayNameInput.lang';

	let { value = $bindable('') }: { value?: string } = $props();

	const handleChange = (next: string) => {
		value = next;
	};
</script>

<TextInput label={locale.displayNameLabel()} {value} onChange={handleChange} />
