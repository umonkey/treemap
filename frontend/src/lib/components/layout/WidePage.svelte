<script lang="ts">
	import { Header } from '$lib/ui';
	import type { Snippet } from 'svelte';

	const { children, title, back, nopadding } = $props<{
		children: Snippet;
		title?: string;
		back?: boolean | string | undefined;
		nopadding?: boolean;
	}>();
</script>

<svelte:head>
	{#if title}
		<title>{title}</title>
	{/if}
</svelte:head>

{#if title}
	<Header {title} {back} />
{/if}

<div class="wide-page" class:padded={!nopadding}>
	{@render children()}
</div>

<style>
	.wide-page {
		&.padded {
			padding: var(--gap);
		}
	}

	@media (max-width: 480px) {
		.wide-page {
			padding-top: var(--gap);
			padding-bottom: var(--gap);

			&.padded {
				padding-left: var(--gap);
				padding-right: var(--gap);
			}
		}
	}
</style>
