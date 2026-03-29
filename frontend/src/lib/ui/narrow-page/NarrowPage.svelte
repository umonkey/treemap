<script lang="ts">
	import type { Snippet } from 'svelte';
	import Header from '../header/Header.svelte';
	import Dialog from '$lib/components/layout/Dialog.svelte';

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

<Dialog title={title}>
	{@render children()}
</Dialog>

<style>
	.dialog {
		position: absolute;

		top: 50%;
		left: 50%;
		max-width: 500px;
		max-height: 80vh;
		transform: translate(-50%, -50%);

		background-color: var(--form-background);
	}

	.narrow-page {
		max-width: 600px;
		margin: 0 auto;
		padding: 0 1rem;
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
