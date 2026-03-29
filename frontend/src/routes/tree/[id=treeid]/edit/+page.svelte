<script lang="ts">
	import EditForm from './EditForm.svelte';
	import Dialog from '$lib/components/layout/Dialog.svelte';

	const { data } = $props();
</script>

<Dialog title="Edit Tree">
	<EditForm id={data.id} />
</Dialog>
