<script lang="ts">
import HelpButton from "$lib/components/forms/HelpButton.svelte";

export const value: string | number = "";
export const placeholder = "";
export const label = "";
export const type = "text";
export const multiline = false;
export const hint = "";
export const help: string | undefined = undefined;
</script>

<div class="input">
	<label>
		<span>{label}</span>
		{#if multiline}
			<textarea bind:value {placeholder}>{value}</textarea>
		{:else}
			<div class="group">
				<input class="form" {type} bind:value {placeholder} />
				<HelpButton {help} />
			</div>
		{/if}
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
