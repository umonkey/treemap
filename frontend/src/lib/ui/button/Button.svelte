<script lang="ts">
	const {
		label,
		type = 'submit',
		onClick,
		link,
		disabled = false
	} = $props<{
		label: string;
		type: 'submit' | 'button' | 'reset';
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
