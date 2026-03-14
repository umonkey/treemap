<script lang="ts">
	import EditForm from './EditForm.svelte';
	import { NarrowPage } from '$lib/ui';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';

	const { data } = $props();
</script>

<NarrowPage>
	<EditForm id={data.id} />
</NarrowPage>
