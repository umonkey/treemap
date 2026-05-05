<script lang="ts">
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import { selectorState } from './LayerSelector.svelte.ts';
	import BASIC from './basic.png';
	import DRONE from './drone.png';
	import LIGHT from './light.png';
</script>

<Dialog title="Select map base layer" variant="bottom">
	<div class="items">
		<div class="item" class:active={selectorState.base == 'basic'}>
			<button
				type="button"
				aria-label="Select Basic base layer"
				onclick={() => selectorState.setBase('basic')}
			>
				<img src={BASIC} alt="Basic layer preview" />
			</button>
			<div class="label">Basic OSM</div>
		</div>

		<div class="item" class:active={selectorState.base == 'light'}>
			<button
				type="button"
				aria-label="Select Light base layer"
				onclick={() => selectorState.setBase('light')}
			>
				<img src={LIGHT} alt="Light layer preview" />
			</button>
			<div class="label">Light</div>
		</div>

		<div class="item" class:active={selectorState.base == 'google'}>
			<button
				type="button"
				aria-label="Select Google Satellite base layer"
				onclick={() => selectorState.setBase('google')}
			>
				<img src={BASIC} alt="Google Satellite layer preview" />
			</button>
			<div class="label">Google</div>
		</div>
	</div>

	<h3>Select additional layers</h3>

	<div class="items">
		<div class="item" class:active={!!selectorState.drone}>
			<button type="button" aria-label="Toggle Drone layer" onclick={selectorState.toggleDrone}>
				<img src={DRONE} alt="Drone layer preview" />
			</button>
			<div class="label">Drone</div>
		</div>
	</div>
</Dialog>

<style>
	.items {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(100px, 1fr));
		gap: 1rem;
		width: 100%;

		@media screen and (max-width: 600px) {
			grid-template-columns: repeat(auto-fit, minmax(80px, 1fr));
			gap: 0.5rem;
		}
	}

	.item {
		width: 100%;

		img {
			width: 100px;
			height: 100px;
			aspect-ratio: 1;
			display: block;
			object-fit: cover;
			margin: 0 auto;
		}

		@media screen and (max-width: 600px) {
			img {
				width: 20vw;
				height: 20vw;
				margin: 0 auto;
			}
		}

		button {
			border: 4px solid transparent;
			background-color: transparent;
			padding: 2px;
			cursor: pointer;
			display: block;
		}

		&.active button,
		&:hover button {
			border-color: green;
		}

		.label {
			text-align: left;
			padding: 0.5rem 0 0 4px;
		}
	}

	h3 {
		font-size: 1.25rem;
		font-weight: 400;
		margin: 2rem 0 0.5rem;
	}
</style>
