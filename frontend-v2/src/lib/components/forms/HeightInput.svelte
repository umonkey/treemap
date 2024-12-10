<script>
	import TextInput from '$lib/components/forms/TextInput.svelte';
	export let value = '';
</script>

<TextInput label="Height, m" bind:value type="number" />
