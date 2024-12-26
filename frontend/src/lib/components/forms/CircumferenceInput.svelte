<script>
	import { locale } from '$lib/locale';
	import TextInput from '$lib/components/forms/TextInput.svelte';
	export let value = '';
</script>

<TextInput
	label={locale.circumferenceLabel()}
	bind:value
	type="number"
	hint={locale.circumferenceHint()}
	help="https://github.com/KanachYerevan/kb/wiki/Measuring-trunk-circumference"
/>
