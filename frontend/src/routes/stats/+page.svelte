<script lang="ts">
	import { routes } from '$lib/routes';
	import { Header, NarrowPage } from '$lib/ui';
</script>

<svelte:head>
	<title>Stats — Trees of Yerevan</title>
</svelte:head>

<Header title="Tree Map" />

<NarrowPage>
	<h2>Data reports</h2>

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
	</ul>

	<h2>Other data</h2>

	<p>
		You can also download the full dataset in SQLite format using <a
			href="https://www.dropbox.com/scl/fo/e28bfrhn73wkfj535lrrj/AFhMhMhG-fq4b5k6RYcW6KU?rlkey=x85yktv1q59aiu2qdnanocllg&st=p8rjna9v&dl=0"
			>this link</a
		>.
	</p>
</NarrowPage>

<style>
	ul {
		line-height: 1.5em;
	}
</style>
