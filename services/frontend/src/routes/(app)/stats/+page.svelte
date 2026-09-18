<script lang="ts">
	import { routes } from '$lib/routes';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import { locale } from '$lib/locale';
</script>

<svelte:head>
	<title>{locale.sideReports()} — {locale.appTitle()}</title>
</svelte:head>

<Dialog title="Data Reports">
	<ul>
		<li><a href={routes.statsSpecies()}>Top species</a></li>
		<li><a href={routes.statsStreets()}>Top streets</a></li>
		<li><a href={routes.statsHeight()}>Top by height</a></li>
		<li><a href={routes.statsDiameter()}>Top by diameter</a></li>
		<li><a href={routes.statsCircumference()}>Top by trunk circumference</a></li>
		<li><a href={routes.statsState()}>Group by state</a></li>
		<li><a href={routes.streetReport()}>Street report</a></li>
	</ul>

	<h2>Moderation queue</h2>

	<ul>
		<li><a href={routes.statsMismatch()}>Trees with unknown species</a></li>
		<li><a href="/duplicates">Trees with similar coordinates</a></li>
	</ul>
</Dialog>

<style>
	ul {
		line-height: 1.5em;
	}
</style>
