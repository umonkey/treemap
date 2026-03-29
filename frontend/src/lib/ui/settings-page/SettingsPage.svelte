<script lang="ts">
	import { SettingsForm } from '$lib/ui';
	import { locale } from '$lib/locale';
	import Dialog from '$lib/components/layout/Dialog.svelte';
</script>

<Dialog title={locale.settingsTitle()}>
	<SettingsForm />
</Dialog>
