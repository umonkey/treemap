<script>
	import TextInput from '$lib/components/forms/TextInput.svelte';
	export let value = '';
</script>

<TextInput label="Canopy diameter, m" bind:value type="number" />
