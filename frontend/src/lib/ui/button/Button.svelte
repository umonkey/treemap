<script lang="ts">
	import '$lib/styles/colors.css';

	const {
		label,
		type = 'submit',
		onClick,
		link,
		disabled = false
	} = $props<{
		label: string;
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
		{label}
	</a>
{:else}
	<button type="button" disabled={!!disabled} class={className} onclick={onClick}>{label}</button>
{/if}

<style>
	.button {
		background-color: var(--form-border);
		border: 1px solid var(--form-border);
		color: var(--form-background);
		outline: none;
		padding: 0.5rem 1rem;
		cursor: pointer;
		border-radius: 6px;
		text-decoration: none;
		font-family: inherit;
		font-size: 14px;
		box-sizing: border-box;
		display: inline-block;

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
