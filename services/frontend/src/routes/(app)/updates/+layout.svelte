<script lang="ts">
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import { locale } from '$lib/locale';
	import Tabs from './Tabs.svelte';

	const { children } = $props();
</script>

<Dialog title={locale.sideUpdates()}>
	<Tabs />
	{@render children()}
</Dialog>
