<script lang="ts">
	import { getStreetReportCSV } from '$lib/api/streets';
	import { routes } from '$lib/routes';
	import type { StreetReport } from '$lib/types';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import HeightReport from '$lib/ui/height-report/HeightReport.svelte';
	import SpeciesReport from '$lib/ui/species-report/SpeciesReport.svelte';
	import StateReport from '$lib/ui/state-report/StateReport.svelte';

	type Props = {
		data: StreetReport;
	};

	const { data }: Props = $props();
</script>

<div class="report">
	<h2>Report: {data.street}</h2>
	<p>
		There are {data.existing} trees, providing {Math.round(data.total_shade)} m² of shade, average {data.average_shade.toFixed(
			1
		)} m² per tree.
	</p>

	<Buttons>
		<Button link={routes.searchAddress(data.street)}>Open map</Button>
		<Button link={getStreetReportCSV(data.street)}>Download CSV</Button>
	</Buttons>

	<StateReport data={data.states} />
	<HeightReport data={data.heights} title="Trees by height" />
	<HeightReport data={data.crowns} title="Trees by crown diameter" />
	<HeightReport data={data.girths} title="Trees by trunk girth" />
	<SpeciesReport data={data.species} street={data.street} total={data.total} />
</div>
