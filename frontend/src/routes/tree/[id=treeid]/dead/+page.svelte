<script lang="ts">
	import { locale } from '$lib/locale';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import TreeDeadForm from './TreeDeadForm.svelte';

	const { data } = $props<{
		data: {
			id: string;
		};
	}>();
</script>

<Dialog title={locale.deadTitle()}>
	<TreeDeadForm id={data.id} />
</Dialog>
