<script lang="ts">
	import { NarrowPage } from '$lib/ui';
	import AddRowForm from './AddRowForm.svelte';
	import { locale } from '$lib/locale';

	import type { ILatLng } from '$lib/types';

	const { data } = $props<{
		data: {
			start: ILatLng;
			end: ILatLng;
		};
	}>();
</script>

<svelte:head>
	<title>{locale.addRowTitle()}</title>
</svelte:head>

<NarrowPage>
	<AddRowForm start={data.start} end={data.end} />
</NarrowPage>
