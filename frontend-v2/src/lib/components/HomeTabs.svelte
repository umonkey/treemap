<script>
	import { routes } from '$lib/routes';
	import TabRenderer from '$lib/components/TabRenderer.svelte';

	export let active;

	const tabs = [
		{
			id: 'welcome',
			title: 'Welcome',
			url: routes.home()
		},
		{
			id: 'stats',
			title: 'Stats',
			url: routes.stats()
		}
	];
</script>

<TabRenderer {active} {tabs} />
