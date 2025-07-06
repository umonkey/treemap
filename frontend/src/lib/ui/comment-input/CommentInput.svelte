<script lang="ts">
	import { TextInput } from '$lib/ui';
	import { locale } from '$lib/locale';

	const { value, onChange, onKeyDown } = $props<{
		value: string;
		onChange: (value: string) => void;
		onKeyDown?: (event: KeyboardEvent) => void;
	}>();

	const handleChange = (value: string) => {
		onChange(value);
	};
</script>

<TextInput
	label={locale.commentLabel()}
	{value}
	multiline={true}
	hint={locale.commentHint()}
	onChange={handleChange}
	onKeyDown={onKeyDown}
/>
