<script lang="ts">
	import { locale } from '$lib/locale';
	import MoveForm from './MoveForm.svelte';
	import Dialog from '$lib/components/layout/Dialog.svelte';

	const { data } = $props();
</script>

<Dialog title={locale.measureTitle()}>
	<MoveForm id={data.id} />
</Dialog>
