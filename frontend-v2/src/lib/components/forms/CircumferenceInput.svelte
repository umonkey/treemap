<script>
	import TextInput from '$lib/components/forms/TextInput.svelte';
	export let value = '';
</script>

<TextInput label="Trunk circumference, cm" bind:value type="number" />
