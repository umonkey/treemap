<script lang="ts">
	import { HelpButton } from '$lib/ui';
	import '$lib/styles/forms.css';

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

<div class="input form">
	<label>
		<span>{label}</span>
		<div class="group">
			<!-- svelte-ignore a11y_autofocus -->
			<input class="form" type="number" {value} {placeholder} {autofocus} onchange={handleChange} />
			{#if help}
				<HelpButton {help} />
			{/if}
		</div>
	</label>

	{#if hint}
		<div class="hint">{hint}</div>
	{/if}
</div>

<style>
	.group {
		display: flex;
		flex-direction: row;
		gap: var(--gap);
	}
</style>
