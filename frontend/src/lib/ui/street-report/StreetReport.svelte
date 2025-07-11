<script lang="ts">
	import type { StreetReport } from '$lib/types';
	import { StateReport, HeightReport } from '$lib/ui';

	type Props = {
		data: StreetReport;
	};

	const { data }: Props = $props();
</script>

<div class="report">
	<h2>Report: {data.street}</h2>
	<p>Total trees: {data.total}</p>

	<StateReport data={data.states} />
	<HeightReport data={data.heights} title="Trees by height" />
	<HeightReport data={data.crowns} title="Trees by crown diameter" />
	<HeightReport data={data.griths} title="Trees by trunk grith" />
</div>
