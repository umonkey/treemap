<script lang="ts">
	import { page } from '$app/stores';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import { rewriteHash } from '$lib/utils/rewrite';
	import { ModeSwitcher } from '$lib/ui';
	import GlobalHeatMap from '$lib/components/GlobalHeatMap/index.svelte';

	import AppInstallButton from '$lib/components/AppInstallButton.svelte';
	import { Header } from '$lib/ui';

	const { data } = $props();
	const { totalCount } = data;

	// Redirect old urls from v1.
	rewriteHash($page.url.hash);
</script>
