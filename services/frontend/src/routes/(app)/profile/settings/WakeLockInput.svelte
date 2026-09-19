<script lang="ts">
	import { wakeLockState } from '$lib/components/screen-lock/state.svelte';
	import { isWakeLockSupported } from '$lib/components/screen-lock/utils';
	import { locale } from '$lib/locale';
	import CheckInput from '$lib/ui/check-input/CheckInput.svelte';
</script>

{#if isWakeLockSupported()}
	<CheckInput
		label={locale.settingsKeepAwake()}
		value={wakeLockState.enabled}
		onChange={(v) => (wakeLockState.enabled = v)}
	/>
{/if}
