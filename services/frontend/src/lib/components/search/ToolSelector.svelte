<script lang="ts">
	import CameraIcon from '$lib/icons/CameraIcon.svelte';
	import CheckListIcon from '$lib/icons/CheckListIcon.svelte';
	import CircumferenceIcon from '$lib/icons/CircumferenceIcon.svelte';
	import DiameterIcon from '$lib/icons/DiameterIcon.svelte';
	import HeightIcon from '$lib/icons/HeightIcon.svelte';
	import ShowIcon from '$lib/icons/ShowIcon.svelte';
	import FormElement from '$lib/ui/form-element/FormElement.svelte';
	import { locale } from './ToolSelector.lang';
	import { ToolSelector } from './ToolSelector.svelte.ts';

	const componentState = new ToolSelector();

	$effect(() => componentState.init());

	const icons = [ShowIcon, HeightIcon, DiameterIcon, CircumferenceIcon, CheckListIcon, CameraIcon];
</script>

<FormElement label={locale.operationMode()} hint={componentState.activeHint}>
	<div class="tool-selector">
		{#each componentState.tools as tool, i}
			{@const Icon = icons[i]}
			<button
				type="button"
				class:active={componentState.activeTool === tool.id}
				onclick={() => componentState.selectTool(tool.id)}
				aria-pressed={componentState.activeTool === tool.id}
				aria-label={tool.label}
				title={tool.label}
			>
				<Icon />
			</button>
		{/each}
	</div>
</FormElement>

<style>
	.tool-selector {
		display: flex;
		width: fit-content;
		margin: 0;
		gap: 4px;
		padding: 4px;
		background: var(--sep-color);
		border-radius: 999px;
		box-shadow: 0 0 0 2px rgba(0, 0, 0, 0.1);
		color: var(--text-color);
	}

	.tool-selector button {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		border: none;
		border-radius: 50%;
		background: transparent;
		cursor: pointer;
	}

	.tool-selector button :global(svg) {
		width: 24px;
		height: 24px;
	}

	.tool-selector button.active {
		background-color: #2e7d32;
		color: #fff;
	}
</style>
