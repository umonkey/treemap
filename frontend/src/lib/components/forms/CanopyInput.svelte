<script>
	import { locale } from '$lib/locale';
	import TextInput from '$lib/components/forms/TextInput.svelte';
	export let value = '';
</script>

<TextInput
	label={locale.canopyLabel()}
	bind:value
	type="number"
	help="https://github.com/KanachYerevan/kb/wiki/Measuring-tree-crown"
/>
