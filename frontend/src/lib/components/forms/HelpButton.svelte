<script lang="ts">
	import HelpIcon from '$lib/icons/HelpIcon.svelte';

	export const { help } = $props();

	const handleHelp = () => {
		if (help) {
			window.open(help, '_blank');
		}
	};
</script>

{#if help}
	<button type="button" onclick={handleHelp} title="Open help page for this value">
		<HelpIcon />
	</button>
{/if}

<style>
	button {
		flex-basis: 30px;
		flex-shrink: 0;
		flex-grow: 0;
		background: transparent;
		border: none;
		color: var(--icon-color-secondary);
		cursor: pointer;
	}
</style>
