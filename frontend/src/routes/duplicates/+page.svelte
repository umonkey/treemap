<script lang="ts">
	import { locale } from '$lib/locale';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import Duplicates from '$lib/pages/duplicates/Duplicates.svelte';
</script>

<Dialog title={locale.searchTitle()}>
	<Duplicates />
</Dialog>
