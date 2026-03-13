<script lang="ts">
	import type { Snippet } from 'svelte';
	import Header from '../header/Header.svelte';

	const { children, title, back, nopadding } = $props<{
		children: Snippet;
		title?: string;
		back?: string;
		nopadding?: boolean;
	}>();
</script>

<svelte:head>
	{#if title}
		<title>{title}</title>
	{/if}
</svelte:head>

{#if title || back}
	<Header {title} {back} />
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

	/** On phones, we have a sticky header. **/
	@media screen and (max-width: 600px) {
		.narrow-page {
			margin-top: 30px;
		}
	}
</style>
