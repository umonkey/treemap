<script lang="ts">
	import EditForm from './EditForm.svelte';
	import { NarrowPage } from '$lib/ui';

	const { data } = $props();
</script>

<NarrowPage>
	<EditForm id={data.id} />
</NarrowPage>
