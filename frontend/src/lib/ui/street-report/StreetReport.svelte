<script lang="ts">
	import type { StreetReport } from '$lib/types';
	import { StateReport, HeightReport, SpeciesReport } from '$lib/ui';

	type Props = {
		data: StreetReport;
	};

	const { data }: Props = $props();
</script>

<div class="report">
	<h2>Report: {data.street}</h2>
	<p>There are {data.total} trees, providing {data.area} m² of shadow.</p>

	<StateReport data={data.states} />
	<HeightReport data={data.heights} title="Trees by height" />
	<HeightReport data={data.crowns} title="Trees by crown diameter" />
	<HeightReport data={data.griths} title="Trees by trunk grith" />
	<SpeciesReport data={data.species} />
</div>
