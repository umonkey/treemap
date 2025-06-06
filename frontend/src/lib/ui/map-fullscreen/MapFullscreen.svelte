<script lang="ts">
	import { onMount } from 'svelte';
	import { hooks } from './hooks';
	import { MapButton } from '$lib/ui';
	import ICON from '$lib/assets/fullscreen.svg';

	const { handleClick } = hooks({ onMount });
</script>

<MapButton position="bottomright" icon={ICON} onClick={handleClick} />
