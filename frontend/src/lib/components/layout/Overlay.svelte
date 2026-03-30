<script lang="ts">
	import type { Snippet } from 'svelte';

	const { children, onClick } = $props<{
		children: Snippet;
		onClick?: () => void;
	}>();
</script>

<div class="overlay">
	{#if onClick}
		<button class="backdrop" onclick={onClick} aria-label="Dismiss"></button>
	{/if}

	{@render children()}
</div>

<style>
	.overlay {
		position: absolute;
		z-index: var(--z-dialog);

		top: 0;
		left: 0;
		right: 0;
		bottom: 0;

		background: rgba(0, 0, 0, 0.5);
		backdrop-filter: blur(2px);
		overflow: hidden;
	}

	.backdrop {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		width: 100%;
		height: 100%;
		background: transparent;
		border: none;
		cursor: pointer;
		padding: 0;
		margin: 0;
	}
</style>
