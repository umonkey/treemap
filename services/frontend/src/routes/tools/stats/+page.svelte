<script lang="ts">
	import { routes } from '$lib/routes';
	import PageHeader from '$lib/ui/header/PageHeader.svelte';
	import { locale } from '$lib/locale';
</script>

<svelte:head>
	<title>{locale.sideReports()} — {locale.appTitle()}</title>
</svelte:head>

<article>
	<PageHeader text="Data Reports" />
	<ul>
		<li><a href={routes.statsSpecies()}>Top species</a></li>
		<li><a href={routes.statsStreets()}>Top streets</a></li>
		<li><a href={routes.statsHeight()}>Top by height</a></li>
		<li><a href={routes.statsDiameter()}>Top by diameter</a></li>
		<li><a href={routes.statsCircumference()}>Top by trunk circumference</a></li>
		<li><a href={routes.statsState()}>Group by state</a></li>
		<li><a href={routes.streetReport()}>Street report</a></li>
	</ul>
</article>

<style>
	ul {
		line-height: 1.5em;
	}
</style>
