<script>
import TextInput from "$lib/components/forms/TextInput.svelte";
export const value = "";
</script>

<TextInput
	label="Address"
	bind:value
	hint="This is normally set automatically when adding a tree, but you can edit."
/>
