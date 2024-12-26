<script>
	import TextInput from '$lib/components/forms/TextInput.svelte';
	import { locale } from '$lib/locale';

	let { value = $bindable('') } = $props();
</script>

<TextInput
	label={locale.yearLabel()}
	bind:value
	type="number"
	help="https://github.com/KanachYerevan/kb/wiki/Year-Planted"
/>
