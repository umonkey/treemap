<script>
	import TextInput from '$lib/components/forms/TextInput.svelte';

	let { value = $bindable('') } = $props();
</script>

<TextInput
	label="Year planted"
	bind:value
	type="number"
	help="https://github.com/KanachYerevan/kb/wiki/Year-Planted"
/>
