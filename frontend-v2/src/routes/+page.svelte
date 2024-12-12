<script>
	import { routes } from '$lib/routes';

	const { data } = $props();
	const { totalCount } = data;
</script>

<svelte:head>
	<title>Trees of Yerevan</title>
</svelte:head>

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
