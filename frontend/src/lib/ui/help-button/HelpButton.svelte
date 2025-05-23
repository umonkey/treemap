<script lang="ts">
	import { HelpIcon } from '$lib/icons';

	const { help } = $props<{
		help: string;
	}>();

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

		max-width: 30px;

		background: transparent;
		border: none;
		color: var(--icon-color-secondary);
		cursor: pointer;
	}
</style>
