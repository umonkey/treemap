<script lang="ts">
	import { locale } from '$lib/locale';
	import ReplaceForm from './ReplaceForm.svelte';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import { routes } from '$lib/routes';

	const { data } = $props();
</script>

<Dialog title={locale.replaceTitle()}>
	<ReplaceForm id={data.id} />
</Dialog>
