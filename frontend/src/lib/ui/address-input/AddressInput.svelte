<script lang="ts">
	import { TextInput } from '$lib/ui';

	const { value, onChange } = $props<{
		value: string;
		onChange: (value: string) => void;
	}>();
</script>

<div class="form">
	<TextInput
		label="Address"
		{value}
		hint="This is normally set automatically when adding a tree, but you can edit."
		{onChange}
	/>
</div>
