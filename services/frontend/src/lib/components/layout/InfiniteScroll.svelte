<script lang="ts">
	import type { Snippet } from 'svelte';

	const ROOT_MARGIN = '100px';

	interface Props {
		children?: Snippet;
		onLoadMore: () => void;
		enabled?: boolean;
	}

	let { children, onLoadMore, enabled = true }: Props = $props();

	let sentinel: HTMLDivElement | undefined = $state();

	$effect(() => {
		if (!sentinel || !enabled) {
			return;
		}

		const observer = new IntersectionObserver(
			(entries) => {
				if (entries[0].isIntersecting) {
					onLoadMore();
				}
			},
			{ rootMargin: ROOT_MARGIN }
		);

		observer.observe(sentinel);

		return () => {
			observer.disconnect();
		};
	});
</script>

{@render children?.()}
<div bind:this={sentinel} class="sentinel"></div>

<style>
	.sentinel {
		height: 1px;
		width: 100%;
		visibility: hidden;
	}
</style>
