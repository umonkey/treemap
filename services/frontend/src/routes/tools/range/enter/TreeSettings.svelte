<script lang="ts">
	import { TreeSettingsState } from './TreeSettings.svelte.ts';
	import { lang } from './enterLang';
	import Button from '$lib/ui/button/Button.svelte';

	const state = new TreeSettingsState();
</script>

{#if state.selectedTreeId && state.selectedTree}
	<div class="tree-settings">
		<span class="info">Tree #{state.selectedIndex + 1}</span>
		<div class="actions">
			<Button type="secondary" onClick={state.remove}>{lang.removeTree}</Button>
			<Button onClick={state.keep}>{lang.keep}</Button>
		</div>
	</div>
{/if}

<style>
	.tree-settings {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0.75rem 1rem;
		background: var(--color-dialog-background, #fff);
		border: 1px solid var(--pico-muted-border-color, #ccc);
		border-radius: 8px;
		gap: 1rem;

		.info {
			font-size: 0.95rem;
			font-weight: 500;
			font-family: monospace;
		}

		.actions {
			display: flex;
			gap: 0.5rem;
		}
	}
</style>
