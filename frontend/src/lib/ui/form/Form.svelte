<script lang="ts">
	import type { Snippet } from 'svelte';

	const { children, onSubmit } = $props<{
		children: Snippet;
		onSubmit?: () => void;
	}>();

	const handleSubmit = () => {
		if (onSubmit) {
			onSubmit();
		}
	};
</script>

<form onsubmit={handleSubmit}>
	{@render children()}
</form>

<style>
	form {
		display: flex;
		flex-direction: column;
		gap: calc(2 * var(--gap));
	}
</style>
