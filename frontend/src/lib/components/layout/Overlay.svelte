<script lang="ts">
	import type { Snippet } from 'svelte';

	const { children, onClick } = $props<{
		children: Snippet;
		onClick?: () => void;
	}>();
</script>

<div class="overlay" onclick={onClick}>
	{@render children()}
</div>

<style>
	.overlay {
		position: absolute;
		z-index: 100;

		top: 0;
		left: 0;
		right: 0;
		bottom: 0;

		background: rgba(0, 0, 0, 0.5);
		backdrop-filter: blur(2px);
		overflow: hidden;
	}
</style>
