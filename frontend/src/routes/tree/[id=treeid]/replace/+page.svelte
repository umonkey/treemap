<script lang="ts">
	import ReplaceForm from './ReplaceForm.svelte';

	const { data } = $props();
</script>

<ReplaceForm id={data.id} />
