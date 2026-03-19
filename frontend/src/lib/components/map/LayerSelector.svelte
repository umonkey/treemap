<script lang="ts">
	import BASIC from './basic.png';
	import LIGHT from './light.png';
	import DRONE from './drone.png';

	import { selectorState } from './LayerSelector.svelte.ts';
</script>

<h3>Select map base layer</h3>

<div class="items">
	<div class="item" class:active={selectorState.base == 'basic'}>
		<button type="button" onclick={() => selectorState.setBase('basic')}>
			<img src={BASIC} />
		</button>
		<div class="label">Basic</div>
	</div>

	<div class="item" class:active={selectorState.base == 'light'}>
		<button type="button" onclick={() => selectorState.setBase('light')}>
			<img src={LIGHT} />
		</button>
		<div class="label">Light</div>
	</div>

	<div class="item" class:active={selectorState.base == 'dark'}>
		<button type="button" onclick={() => selectorState.setBase('dark')}>
			<img src={LIGHT} />
		</button>
		<div class="label">Dark</div>
	</div>
</div>

<h3>Select additional layers</h3>

<div class="items">
	<div class="item" class:active={!!selectorState.drone}>
		<button type="button" onclick={selectorState.toggleDrone}>
			<img src={DRONE} />
		</button>
		<div class="label">Drone</div>
	</div>
</div>

<style>
	.items {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 1rem;
		width: 100%;
	}

	.item {
		width: 100%;

		img {
			width: 100%;
			display: block;
			object-fit: none;
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
			text-align: center;
			padding: 0.5rem 0;
		}
	}

	h3 {
		font-size: 1.25rem;
		font-weight: 400;
		margin: 2rem 0 0.5rem;
	}
</style>
