<script lang="ts">
	import BarsIcon from '$lib/icons/BarsIcon.svelte';
	import { Control } from 'svelte-maplibre';
	import { componentState } from './BarsButton.svelte.ts';
</script>

<Control position="bottom-left">
	<div class="maplibregl-ctrl-group mobile-only">
		<button type="button" onclick={componentState.openSidebar} title="Open Menu">
			<BarsIcon />
		</button>
	</div>
</Control>

<style>
	button {
		padding: 4px;
		color: #000;
		width: 29px;
		height: 29px;
		display: block;
	}

	.mobile-only {
		display: block;
	}

	@media (min-width: 1024px) {
		.mobile-only {
			display: none;
		}
	}
</style>
