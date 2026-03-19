<script lang="ts">
	import Icon from '$lib/icons/Layers.svelte';
	import LayerSelector from './LayerSelector.svelte';
	import Button from '$lib/ui/button/Button.svelte';
	import { Control } from 'svelte-maplibre';
	import { buttonState } from './LayerButton.svelte.ts';
</script>

<Control position="top-right">
	<div class="maplibregl-ctrl-group" class:active={buttonState.active}>
		<button type="button" title="Switch layers" onclick={buttonState.toggle}>
			<Icon />
		</button>
	</div>
</Control>

{#if buttonState.active}
	<div class="overlay" onclick={buttonState.toggle}>
		<div class="canvas" onclick={buttonState.ignoreClick}>
			<LayerSelector />

			<div class="buttons">
				<Button onClick={buttonState.close}>Close</Button>
			</div>
		</div>
	</div>
{/if}

<style>
	button {
		padding: 4px;
		color: #000;
	}

	.active {
		background-color: rgba(0 128 0 / 0.5);
	}

	.overlay {
		position: fixed;
		top: 0;
		left: 0;
		height: 100%;
		width: 100%;
		background-color: rgba(0, 0, 0, 0.5);
		backdrop-filter: blur(2px);
		z-index: 100;
	}

	.buttons {
		margin-top: 2rem;
	}

	.canvas {
		background-color: var(--form-background);
		max-width: 500px;
		padding: 0 1rem 1rem;
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
	}

	@media screen and (max-width: 600px) {
		.canvas {
			top: 0;
			left: 0;
			height: 100vh;
			width: 100vw;
			transform: none;
			box-sizing: border-box;
		}
	}
</style>
