<script lang="ts">
	import type { Snippet } from 'svelte';
	import { hooks } from './hooks';

	type Props = {
		children: Snippet;
		sticky?: boolean;
		onSubmit?: () => void;
	};

	const { children, onSubmit, sticky = false }: Props = $props();

	const { handleSubmit, handleKeyDown } = hooks({ onSubmit });
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<form onsubmit={handleSubmit} onkeydown={handleKeyDown} class:sticky={!!sticky}>
	{@render children()}
</form>

<style>
	form {
		display: flex;
		flex-direction: column;
		gap: calc(2 * var(--gap));
	}

	@media screen and (max-width: 600px) {
		.sticky {
			padding-bottom: 60px;
		}
	}
</style>
