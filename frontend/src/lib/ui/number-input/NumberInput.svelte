<script lang="ts">
	import { FormElement } from '$lib/ui';

	const {
		value,
		placeholder = undefined,
		label = undefined,
		hint = undefined,
		help = undefined,
		autofocus,
		onChange = () => {}
	} = $props<{
		value: number | null;
		placeholder?: string | undefined;
		label?: string | undefined;
		hint?: string | undefined;
		help?: string | undefined;
		autofocus?: boolean;
		onChange: (value: number) => void;
	}>();

	const handleChange = (e: Event) => {
		if (e.target) {
			onChange((e.target as HTMLInputElement).valueAsNumber);
		}
	};
</script>

<FormElement {label} {hint} {help}>
	<!-- svelte-ignore a11y_autofocus -->
	<input
		class="form-element"
		type="number"
		{value}
		{placeholder}
		{autofocus}
		onchange={handleChange}
	/>
</FormElement>
