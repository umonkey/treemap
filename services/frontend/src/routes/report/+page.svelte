<script lang="ts">
	import { page } from '$app/stores';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import { locale } from '$lib/locale';
	import Form from '$lib/ui/form/Form.svelte';
	import StreetInput from '$lib/ui/street-input/StreetInput.svelte';
	import { pageState } from './page.svelte';

	import { getStreetReportCSV } from '$lib/api/streets';
	import { routes } from '$lib/routes';
	import Button from '$lib/ui/button/Button.svelte';
	import Buttons from '$lib/ui/buttons/Buttons.svelte';
	import HeightReport from '$lib/ui/height-report/HeightReport.svelte';
	import SpeciesReport from '$lib/ui/species-report/SpeciesReport.svelte';
	import StateReport from '$lib/ui/state-report/StateReport.svelte';

	const address = $derived($page.url.searchParams.get('address'));

	$effect(() => {
		pageState.reload(address ?? null);
	});
</script>

<Dialog title={locale.reportTitle()}>
	<div class="report">
		<Form>
			<StreetInput value={address} onChange={pageState.handleStreetChange} />
		</Form>
	</div>

	{#if pageState.report}
		<div class="report">
			<h2>Report: {pageState.report.street}</h2>
			<p>
				There are {pageState.report.existing} trees, providing {Math.round(
					pageState.report.total_shade
				)} m² of shade, average {pageState.report.average_shade.toFixed(1)} m² per tree.
			</p>

			<Buttons>
				<Button link={routes.searchAddress(pageState.report.street)}>Open map</Button>
				<Button link={getStreetReportCSV(pageState.report.street)}>Download CSV</Button>
			</Buttons>

			<StateReport data={pageState.report.states} />
			<HeightReport data={pageState.report.heights} title="Trees by height" />
			<HeightReport data={pageState.report.crowns} title="Trees by crown diameter" />
			<HeightReport data={pageState.report.girths} title="Trees by trunk girth" />
			<SpeciesReport
				data={pageState.report.species}
				street={pageState.report.street}
				total={pageState.report.total}
			/>
		</div>
	{/if}
</Dialog>

<style>
	.report {
		padding: 0 var(--gap);
	}
</style>
