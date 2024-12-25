<script>
	import TextInput from '$lib/components/forms/TextInput.svelte';
	export let value = '';
</script>

<TextInput
	label="Description"
	bind:value
	multiline={true}
	hint="Add some notable details that could help visitors understand the importance of this tree."
/>
