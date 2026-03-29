<script lang="ts">
	import type { Snippet } from 'svelte';
	import Overlay from '$lib/components/layout/Overlay.svelte';

	const { title, children } = $props<{
		title: string;
		children: Snippet;
	}>();
</script>

<Overlay>
	<div class="dialog">
		<div class="title">
			<h1>{title}</h1>
		</div>

		<div class="body">
			{@render children()}
		</div>
	</div>
</Overlay>

<style>
	.dialog {
		position: absolute;

		top: 50%;
		left: 50%;
		max-width: 500px;
		transform: translate(-50%, -50%);

		background-color: var(--background-color);
		border-radius: 10px;
		overflow: hidden;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
	}

	.title {
		padding: 0.5rem 1rem;
		background-color: rgba(0, 0, 0, 0.2);
	}

	h1 {
		font-size: 1.25rem;
		font-weight: 400;
		line-height: 1.5rem;
		text-align: center;
		margin: 0;
	}

	.body {
		max-width: 600px;
		margin: 0 auto;
		padding: 1rem;

		max-height: 80vh;
		overflow-x: hidden;
		overflow-y: scroll;

		display: flex;
		flex-direction: column;
		gap: 1rem;
	}
</style>
