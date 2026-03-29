<script lang="ts">
	import { locale } from '$lib/locale';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import DeleteTreeForm from './DeleteTreeForm.svelte';

	const { data } = $props<{
		data: {
			id: string;
		};
	}>();
</script>

<Dialog title={locale.deleteTreeTitle()}>
	<DeleteTreeForm id={data.id} />
</Dialog>
