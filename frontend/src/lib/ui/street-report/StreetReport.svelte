<script lang="ts">
	import type { StreetReport } from '$lib/types';
	import { routes } from '$lib/routes';
	import { StateReport, HeightReport, SpeciesReport } from '$lib/ui';

	type Props = {
		data: StreetReport;
	};

	const { data }: Props = $props();
</script>

<div class="report">
	<h2>Report: {data.street}</h2>
	<p>
		There are {data.total} trees, providing {data.area} m² of shadow, average {(
			data.area / data.total
		).toFixed(1)} m² per tree.
	</p>
	<p><a href={routes.searchAddress(data.street)}>Open trees on the map</a>.</p>

	<StateReport data={data.states} />
	<HeightReport data={data.heights} title="Trees by height" />
	<HeightReport data={data.crowns} title="Trees by crown diameter" />
	<HeightReport data={data.griths} title="Trees by trunk grith" />
	<SpeciesReport data={data.species} street={data.street} />
</div>
