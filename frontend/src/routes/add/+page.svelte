<script lang="ts">
	import AddForm from './AddForm.svelte';

	const { data } = $props<{
		data: {
			lat: number;
			lng: number;
		};
	}>();
</script>

<AddForm lat={data.lat} lng={data.lng} />
