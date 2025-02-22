<script lang="ts">
	import HelpButton from '$lib/components/forms/HelpButton.svelte';

	export let value: string | number = '';
	export let placeholder = '';
	export let label = '';
	export let type = 'text';
	export let multiline = false;
	export let hint = '';
	export let help: string | undefined = undefined;
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
