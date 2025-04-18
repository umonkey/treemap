<script lang="ts">
	import { TextInput } from '$lib/ui';
	import { locale } from '$lib/locale';

	const { value, onChange } = $props<{
		value: string;
		onChange: (value: string) => void;
	}>();
</script>

<TextInput
	label={locale.notesLabel()}
	{value}
	multiline={true}
	hint={locale.notesHint()}
	{onChange}
/>
