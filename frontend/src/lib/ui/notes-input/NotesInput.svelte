<script lang="ts">
	import { locale } from '$lib/locale';
	import { TextInput } from '$lib/ui';

	const { value, onChange } = $props<{
		value: string | null;
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
