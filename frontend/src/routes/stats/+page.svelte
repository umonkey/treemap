<script lang="ts">
import { routes } from "$lib/routes";

import Header from "$lib/components/tree/Header.svelte";
</script>

<svelte:head>
	<title>Stats — Trees of Yerevan</title>
</svelte:head>

<Header title="Tree Map" />

<div class="padded">
	<h1>Data reports</h1>

	<ul>
		<li><a href={routes.statsSpecies()}>Top species</a></li>
		<li><a href={routes.statsStreets()}>Top streets</a></li>
		<li><a href={routes.statsHeight()}>Top by height</a></li>
		<li><a href={routes.statsDiameter()}>Top by diameter</a></li>
		<li><a href={routes.statsCircumference()}>Top by trunk circumference</a></li>
		<li><a href={routes.statsState()}>Group by state</a></li>
	</ul>

	<h1>Moderation queue</h1>

	<ul>
		<li><a href={routes.statsMismatch()}>Trees with unknown species</a></li>
	</ul>
</div>

<style>
	ul {
		line-height: 1.5em;
	}
</style>
