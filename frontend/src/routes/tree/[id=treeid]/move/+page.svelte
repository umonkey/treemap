<script lang="ts">
	import MoveForm from './MoveForm.svelte';

	const { data } = $props();
</script>

<MoveForm id={data.id} />
