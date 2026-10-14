<script lang="ts">
	import GlobalHeatMap from '$lib/components/heat/GlobalHeatMap.svelte';
	import TopUsers from '$lib/components/top-users/TopUsers.svelte';
	import { locale as appLocale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import PageHeader from '$lib/ui/header/PageHeader.svelte';
	import { locale } from './lang';
</script>

<svelte:head>
	<title>{locale.title()} — {appLocale.appTitle()}</title>
</svelte:head>

<article>
	<PageHeader text={locale.header()} />

	<h2>{locale.heatmapHeader()}</h2>
	<GlobalHeatMap />

	<h2>{locale.contributorsHeader()}</h2>
	<TopUsers />

	<h2>{locale.reportsHeader()}</h2>
	<ul>
		<li><a href={routes.statsSpecies()}>{locale.species()}</a></li>
		<li><a href={routes.statsStreets()}>{locale.streets()}</a></li>
		<li><a href={routes.statsHeight()}>{locale.height()}</a></li>
		<li><a href={routes.statsDiameter()}>{locale.diameter()}</a></li>
		<li><a href={routes.statsCircumference()}>{locale.circumference()}</a></li>
		<li><a href={routes.statsState()}>{locale.state()}</a></li>
		<li><a href={routes.streetReport()}>{locale.streetReport()}</a></li>
	</ul>
</article>

<style>
	ul {
		line-height: 1.5em;
	}
</style>
