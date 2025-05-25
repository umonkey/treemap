<script lang="ts">
	import type { Snippet } from 'svelte';
	import { HelpButton } from '$lib/ui';

	const { label, children, help, hint } = $props<{
		children: Snippet;
		label?: string;
		hint?: string;
		help?: string;
	}>();
</script>

<div class="form-element">
	<label>
		{#if label}
			<span>{label}</span>
		{/if}

		<div class="group">
			<div class="inner">
				{@render children()}
			</div>

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
	.form-element {
		display: flex;
		flex-direction: column;
		gap: var(--gap);

		label {
			display: block;

			& > span {
				display: block;
				margin-bottom: var(--gap);
				opacity: 0.75;
			}
		}

		.group {
			display: flex;
			flex-direction: row;
			gap: var(--gap);

			.inner {
				flex-grow: 1;
			}
		}

		:global(input),
		:global(textarea) {
			width: 100%;
			padding: var(--gap);
			border: 1px solid var(--border);
			border-radius: var(--radius);
			box-sizing: border-box;
			background-color: transparent;
			border: 1px solid var(--sep-color);
			border-radius: 6px;
			color: var(--form-color);
			outline: none;
			line-height: 1.25em;
		}

		:global(textarea) {
			min-width: 100%;
			max-width: 100%;
			min-height: 100px;
		}

		.hint {
			color: var(--text-color-inactive);
			font-size: 0.85em;
			line-height: 125%;
			margin: 0;
		}
	}
</style>
