<script lang="ts">
	import EditForm from './EditForm.svelte';

	const { data } = $props();
</script>

<EditForm id={data.id} />
