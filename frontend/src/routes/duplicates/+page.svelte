<script lang="ts">
	import { locale } from '$lib/locale';
	import { NarrowPage } from '$lib/ui';
	import Duplicates from '$lib/pages/duplicates/Duplicates.svelte';
</script>

<NarrowPage title={locale.searchTitle()}>
	<Duplicates />
</NarrowPage>
