<script lang="ts">
	import { locale } from '$lib/locale';
	import CheckInput from '$lib/ui/check-input/CheckInput.svelte';
	import { wakeLockState } from './state.svelte';
	import { isWakeLockSupported } from './utils';
</script>

{#if isWakeLockSupported()}
	<CheckInput
		label={locale.settingsKeepAwake()}
		value={wakeLockState.enabled}
		onChange={(v) => (wakeLockState.enabled = v)}
	/>
{/if}
