<script lang="ts">
import HelpButton from "$lib/components/forms/HelpButton.svelte";

const {
	value,
	placeholder = undefined,
	label = undefined,
	hint = undefined,
	help = undefined,
	onChange = () => {},
}: {
	value: number | null;
	placeholder?: string | undefined;
	label?: string | undefined;
	hint?: string | undefined;
	help?: string | undefined;
	onChange: (value: number) => void;
} = $props();

const handleChange = (e: Event) => {
	if (e.target) {
		onChange((e.target as HTMLInputElement).valueAsNumber);
	}
};
</script>

<div class="input">
	<label>
		<span>{label}</span>
		<div class="group">
			<input class="form" type="number" {value} {placeholder} onchange={handleChange} />
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
