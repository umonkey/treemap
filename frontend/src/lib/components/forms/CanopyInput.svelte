<script>
	import TextInput from '$lib/components/forms/TextInput.svelte';
	export let value = '';
</script>

<TextInput
	label="Canopy diameter, m"
	bind:value
	type="number"
	help="https://github.com/KanachYerevan/kb/wiki/Measuring-tree-crown"
/>
