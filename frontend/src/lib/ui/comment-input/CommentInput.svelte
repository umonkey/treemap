<script lang="ts">
	import { TextInput } from '$lib/ui';
	import { locale } from '$lib/locale';

	const { value, hint, onChange } = $props<{
		value: string;
		hint?: string;
		onChange: (value: string) => void;
	}>();

	const handleChange = (value: string) => {
		onChange(value);
	};
</script>

<TextInput
	label={locale.commentLabel()}
	{value}
	multiline={true}
	hint={hint ?? locale.commentHint()}
	onChange={handleChange}
/>
