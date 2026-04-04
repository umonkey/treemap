<script lang="ts">
	import TextInput from '../text-input/TextInput.svelte';

	const { value, onChange } = $props<{
		value: string;
		onChange: (value: string) => void;
	}>();
</script>

<TextInput
	label="Address"
	{value}
	hint="This is normally set automatically when adding a tree, but you can edit."
	{onChange}
/>
