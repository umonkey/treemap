<script>
	import TextInput from '$lib/components/forms/TextInput.svelte';
	export let value = '';
</script>

<TextInput label="Species" bind:value placeholder="Enter the name of the tree" />
