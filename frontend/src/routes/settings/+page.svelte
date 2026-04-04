<script lang="ts">
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import Tabs from '$lib/components/profile/Tabs.svelte';
	import { locale } from '$lib/locale';
	import SettingsForm from './SettingsForm.svelte';
</script>

<Dialog title={locale.settingsTitle()}>
	<Tabs active="settings" />
	<SettingsForm />
</Dialog>
