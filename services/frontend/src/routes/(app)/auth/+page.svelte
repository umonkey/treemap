<script lang="ts">
	import { pageState } from './page.svelte.ts';
	import { page } from '$app/state';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import { locale } from './lang';

	const token = $derived(page.url.searchParams.get('token'));
	const state = $derived(page.url.searchParams.get('state'));

	const buttons = $derived(
		pageState.error
			? [
					{ title: locale.authTryAgain(), onClick: pageState.handleRetry },
					{ title: locale.authContinueAnonymously(), onClick: pageState.handleContinueAnonymously }
				]
			: undefined
	);

	$effect(() => {
		pageState.init(token, state);
	});

	$effect(() => {
		if (pageState.redirect) {
			window.location.href = pageState.redirect;
		}
	});
</script>

<Dialog title={locale.authTitle()} onCancel={pageState.handleContinueAnonymously} {buttons}>
	{#if pageState.loading}
		<p aria-busy="true">Authenticating...</p>
	{:else if pageState.error}
		<p>{locale.authError()}</p>
	{/if}
</Dialog>
