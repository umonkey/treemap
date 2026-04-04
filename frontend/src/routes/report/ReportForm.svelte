<script lang="ts">
	import Form from '$lib/ui/form/Form.svelte';
	import StreetInput from '$lib/ui/street-input/StreetInput.svelte';
	import StreetReport from '$lib/ui/street-report/StreetReport.svelte';
	import { hooks } from './hooks';

	type Props = {
		address: string | null | undefined;
	};

	const { address }: Props = $props();
	const { report, handleStreetChange, reload } = hooks();

	$effect(() => {
		reload(address ?? null);
	});
</script>

<div class="report">
	<Form>
		<StreetInput value={address} onChange={handleStreetChange} />
	</Form>
</div>

{#if $report}
	<StreetReport data={$report} />
{/if}
