<script lang="ts">
	import ObservationForm from './ObservationForm.svelte';

	const { data } = $props();
	const tree = data.tree;
</script>

<ObservationForm id={tree.id} />
