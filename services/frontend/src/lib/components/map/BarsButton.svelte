<script lang="ts">
	import BarsIcon from '$lib/icons/BarsIcon.svelte';
	import { Control } from 'svelte-maplibre';
	import { componentState } from './BarsButton.svelte.ts';
	import { uploadStore } from '$lib/stores/upload';
</script>

<Control position="bottom-right">
	<div class="maplibregl-ctrl-group mobile-only">
		<button type="button" onclick={componentState.openSidebar} title="Open Menu">
			<BarsIcon />
			{#if $uploadStore.pending > 0}
				<span class="badge">{$uploadStore.pending}</span>
			{/if}
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
		position: relative;
	}

	.badge {
		position: absolute;
		top: 0;
		left: 0;
		transform: translate(-50%, -50%);

		font-size: 10px;
		background-color: #080;
		padding: 0 4px;
		border-radius: 10px;
		color: white;
		pointer-events: none;
		line-height: 14px;
		min-width: 14px;
		text-align: center;
		font-weight: bold;
		border: 1px solid white;
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
