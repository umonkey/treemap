<script lang="ts">
	import { page } from '$app/stores';
	import { locale } from '$lib/locale';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import Tabs from '$lib/components/profile/Tabs.svelte';

	const { children } = $props();

	const activeTab = $derived(
		$page.url.pathname.endsWith('/settings')
			? 'settings'
			: $page.url.pathname.endsWith('/uploads')
				? 'uploads'
				: 'profile'
	);

	const title = $derived(
		activeTab === 'settings'
			? locale.settingsTitle()
			: activeTab === 'uploads'
				? locale.uploadsTitle()
				: locale.profileTitle()
	);
</script>

<Dialog {title}>
	<Tabs active={activeTab} />
	{@render children()}
</Dialog>
