<script lang="ts">
	import { locale } from '$lib/locale';
	import SearchForm from './SearchForm.svelte';
	import Dialog from '$lib/components/layout/Dialog.svelte';
</script>

<Dialog title={locale.searchTitle()}>
	<SearchForm />
</Dialog>
