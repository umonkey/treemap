<script lang="ts">
	import { hooks } from './hooks';
	import { onMount } from 'svelte';
	import { MapButton } from '$lib/ui';
	import ICON from '$lib/assets/locate.svg';

	const { handleClick, lastPosition } = hooks({ onMount });
</script>

<MapButton icon={ICON} position="topleft" onClick={handleClick} disabled={!$lastPosition} />
