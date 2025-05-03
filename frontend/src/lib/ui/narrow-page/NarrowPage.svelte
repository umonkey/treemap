<script lang="ts">
	import { Header } from '$lib/ui';
	import type { Snippet } from 'svelte';

	const { children, title, nopadding } = $props<{
		children: Snippet;
		title?: string;
		nopadding?: boolean;
	}>();
</script>

<svelte:head>
	{#if title}
		<title>{title}</title>
	{/if}
</svelte:head>

{#if title}
	<Header {title} />
{/if}

<div class="narrow-page" class:padded={!nopadding}>
	{@render children()}
</div>

<style>
	.narrow-page {
		max-width: 600px;
		margin: 0 auto;

		&.padded {
			padding: var(--gap) 0;
		}
	}

	@media (max-width: 480px) {
		.narrow-page {
			padding-top: var(--gap);
			padding-bottom: var(--gap);

			&.padded {
				padding-left: var(--gap);
				padding-right: var(--gap);
			}
		}
	}
</style>
