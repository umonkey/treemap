<script lang="ts">
	import { SettingsForm, Header, NarrowPage } from '$lib/ui';
	import { locale } from '$lib/locale';
</script>

<svelte:head>
	<title>{locale.settingsTitle()}</title>
</svelte:head>

<Header title={locale.settingsTitle()} />

<NarrowPage>
	<SettingsForm />
</NarrowPage>
