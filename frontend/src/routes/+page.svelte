<script lang="ts">
	import { routes } from '$lib/routes';
	import { page } from '$app/stores';
	import { rewriteHash } from '$lib/utils/rewrite';

	import Header from '$lib/components/tree/Header.svelte';

	const { data } = $props();
	const { totalCount } = data;

	// Redirect old urls from v1.
	rewriteHash($page.url.hash);
</script>

<svelte:head>
	<title>Trees of Yerevan</title>
</svelte:head>

<Header title="Tree Map" back={false} />

<div class="home">
	<h1>Tree Map of Yerevan</h1>
	<p>
		Welcome to the map of trees in Yerevan.<br />We have <strong>{totalCount} trees</strong> in our
		database.<br />
		You can read about this app
		<a href="https://github.com/KanachYerevan/kb/wiki/Mobile-Application" target="_blank"
			>in our wiki</a
		>.
	</p>

	<p>
		Don't know what to do? Find trees around you that are <a href={routes.searchQuery('incomplete')}
			>missing some data</a
		>
		or <a href={routes.searchQuery('noimage')}>missing photos</a> and help us out!
	</p>

	<p>Please use the navigation bar to access areas of the application.</p>
	<p>
		This is a community driven application that everyone can use and update. You can find the source
		code <a href="https://github.com/umonkey/treemap/" target="_blank">here</a>.
	</p>
</div>

<style>
	.home {
		padding: 0 var(--gap);
	}
</style>
