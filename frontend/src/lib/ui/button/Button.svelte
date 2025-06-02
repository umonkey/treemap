<script lang="ts">
	import type { Snippet } from 'svelte';
	import '$lib/styles/colors.css';

	const {
		children,
		type = 'submit',
		onClick,
		link,
		disabled = false
	} = $props<{
		children: Snippet;
		type: 'submit' | 'button' | 'reset' | 'cancel' | 'secondary' | 'tertiary';
		onClick: () => void;
		disabled?: boolean;
		link?: string;
	}>();

	const className = `button ${type}`;

	const getTarget = () => {
		if (link && link.startsWith('http')) {
			return '_blank';
		}

		return undefined;
	};
</script>

{#if link}
	<a href={link} disabled={!!disabled} class={className} target={getTarget()}>
		{@render children()}
	</a>
{:else}
	<button type="button" disabled={!!disabled} class={className} onclick={onClick}
		>{@render children()}</button
	>
{/if}

<style>
	.button {
		background-color: var(--form-border);
		border: 1px solid var(--form-border);
		color: var(--form-background);
		outline: none;
		cursor: pointer;
		border-radius: 6px;
		text-decoration: none;
		font-family: inherit;
		box-sizing: border-box;
		display: inline-block;

		font-size: 14px;
		line-height: 20px;
		padding: 6px 15px;

		/* SVG icons sometimes expand the button. */
		height: 34px;

		&.cancel {
			background-color: transparent;
			color: var(--form-border);
		}

		&.secondary {
			background-color: transparent;
			color: var(--form-border);
		}

		&.tretiary {
			border-color: rgba(16, 131, 254, 0.2);
			background-color: transparent;
			color: var(--form-border);
			text-decoration: underline;
		}

		&:disabled {
			opacity: 0.5;
		}
	}
</style>
