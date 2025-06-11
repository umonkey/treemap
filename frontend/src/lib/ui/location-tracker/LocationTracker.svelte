<script lang="ts">
	// This component implements location tracking via the Geolocation API.
	// It uses the `navigator.geolocation.watchPosition` method to track the user's location
	// and report it to other components via the locationStore.

	import { onMount } from 'svelte';
	import { hooks } from './hooks';

	hooks({ onMount });
</script>
