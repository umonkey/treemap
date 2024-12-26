<script lang="ts">
	import { routes } from '$lib/routes';
	import { page } from '$app/stores';
	import { rewriteHash } from '$lib/utils/rewrite';
	import { locale } from '$lib/locale';

	import Header from '$lib/components/tree/Header.svelte';

	const { data } = $props();
	const { totalCount } = data;

	// Redirect old urls from v1.
	rewriteHash($page.url.hash);
</script>

<svelte:head>
	<title>Trees of Yerevan</title>
</svelte:head>

<Header title={locale.homeTitle()} back={false} />

<div class="home">
	<h1>{locale.appTitle()}</h1>
	<p>
		Welcome to the community-drive map of trees in Yerevan, Armenia. Here we collect information
		about trees to protect them. We have <strong>{totalCount} trees</strong> in our database. You
		can read about this app
		<a href="https://github.com/KanachYerevan/kb/wiki/Mobile-Application" target="_blank"
			>in our wiki</a
		>.
	</p>

	<p>
		New here? Find trees around you that are <a href={routes.searchQuery('incomplete')}
			>missing some data</a
		>
		or <a href={routes.searchQuery('noimage')}>missing photos</a> and help us out!
	</p>
</div>

<style>
	.home {
		padding: 0 var(--gap);
	}
</style>
