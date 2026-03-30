<script lang="ts">
	import AddRowForm from './AddRowForm.svelte';

	import type { ILatLng } from '$lib/types';

	const { data } = $props<{
		data: {
			start: ILatLng;
			end: ILatLng;
		};
	}>();
</script>

<AddRowForm start={data.start} end={data.end} />
