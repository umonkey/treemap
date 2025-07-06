<script lang="ts">
	import type { Snippet } from 'svelte';
	import { hooks } from './hooks';

	type Props = {
		children: Snippet;
		onSubmit?: () => void;
	};

	const { children, onSubmit }: Props = $props();

	const { handleSubmit, handleKeyDown } = hooks({ onSubmit });
</script>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<form onsubmit={handleSubmit} onkeydown={handleKeyDown}>
	{@render children()}
</form>

<style>
	form {
		display: flex;
		flex-direction: column;
		gap: calc(2 * var(--gap));
	}
</style>
