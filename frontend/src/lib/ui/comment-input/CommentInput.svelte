<script lang="ts">
	import { TextInput } from '$lib/ui';
	import { locale } from '$lib/locale';

	const { value, onChange } = $props<{
		value: string;
		onChange: (value: string) => void;
	}>();

	const handleChange = (e: Event) => {
		if (e.target) {
			onChange((e.target as HTMLInputElement).value);
		}
	};
</script>

<TextInput
	label={locale.commentLabel()}
	{value}
	multiline={true}
	hint={locale.commentHint()}
	onchange={handleChange}
/>
