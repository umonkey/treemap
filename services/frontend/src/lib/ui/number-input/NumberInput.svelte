<script lang="ts">
	import type { Snippet } from 'svelte';
	import { untrack } from 'svelte';
	import FormElement from '$lib/ui/form-element/FormElement.svelte';

	const {
		value,
		placeholder = undefined,
		label = undefined,
		hint = undefined,
		help = undefined,
		min = '0',
		max = undefined,
		autofocus,
		children,
		onChange = () => {}
	}: {
		value: number | null;
		placeholder?: string | undefined;
		label?: string | undefined;
		hint?: string | undefined;
		help?: string | undefined;
		autofocus?: boolean;
		/** Parsed bounds; native range attributes are not meaningful on a text input. */
		min?: string | number;
		max?: string | number;
		/** Accepted for API compatibility; no effect on a text input. */
		step?: string | number;
		children?: Snippet;
		onChange: (value: number) => void;
	} = $props();

	const minValue = untrack(() => Number(min));
	const maxValue = untrack(() => (max === undefined ? null : Number(max)));

	// Deliberate initial snapshot; later prop changes are handled by the
	// `$effect` below. `untrack` avoids the state_referenced_locally warning
	// that ESLint escalates to an error.
	let text = $state(untrack(() => (value === null ? '' : String(value))));

	const parseDecimal = (raw: string): number | null => {
		const normalized = raw.trim().replaceAll(',', '.');

		if (!/^[+-]?(?:\d+(?:\.\d*)?|\.\d+)$/.test(normalized)) {
			return null;
		}

		const parsed = Number(normalized);
		if (!Number.isFinite(parsed)) return null;
		if (parsed < minValue) return null;
		if (maxValue !== null && parsed > maxValue) return null;
		return parsed;
	};

	// Accept externally supplied values, but leave the user's text alone while it
	// already represents the same number (e.g. keep the comma in "4,5").
	$effect(() => {
		const incoming = value;

		untrack(() => {
			if (parseDecimal(text) !== incoming) {
				text = incoming === null ? '' : String(incoming);
			}
		});
	});

	const handleInput = (e: Event) => {
		const target = e.target as HTMLInputElement;
		text = target.value;

		const parsed = parseDecimal(text);
		if (parsed !== null) {
			onChange(parsed);
		}
	};
</script>

<FormElement {label} {hint} {help}>
	<!-- svelte-ignore a11y_autofocus -->
	<input
		class="form-element"
		type="text"
		inputmode="decimal"
		autocomplete="off"
		value={text}
		{placeholder}
		{autofocus}
		oninput={handleInput}
	/>

	{@render children?.()}
</FormElement>
