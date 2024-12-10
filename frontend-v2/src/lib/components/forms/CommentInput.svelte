<script>
	import TextInput from '$lib/components/forms/TextInput.svelte';
	export let value = '';
</script>

<TextInput label="Comment" bind:value multiline={true} hint="Some hints or suggested edits." />
