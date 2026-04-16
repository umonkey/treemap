<script lang="ts">
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import { uploadStore } from '$lib/stores/upload';
	import TabList from '$lib/ui/tab-list/TabList.svelte';

	const { active } = $props<{
		active: string;
	}>();

	const tabs = $derived([
		{
			title: locale.profileTitle(),
			link: routes.profile(),
			active: active === 'profile'
		},
		{
			title: locale.uploadsTitle(),
			link: routes.uploads(),
			badge: $uploadStore.pending > 0 ? $uploadStore.pending : undefined,
			active: active === 'uploads'
		},
		{
			title: locale.settingsTitle(),
			link: routes.settings(),
			active: active === 'settings'
		}
	]);
</script>

<TabList items={tabs} />
