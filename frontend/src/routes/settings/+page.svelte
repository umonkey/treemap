<script lang="ts">
	import { SettingsForm } from '$lib/ui';
	import { locale } from '$lib/locale';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import Tabs from '$lib/components/profile/Tabs.svelte';
</script>

<Dialog title={locale.settingsTitle()}>
	<Tabs active="settings" />
	<SettingsForm />
</Dialog>
