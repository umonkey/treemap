<script lang="ts">
	import { routes } from '$lib/routes';
	import { hooks } from './hooks';
	import { onMount } from 'svelte';

	const { focused, value, handleReset, handleFocus, handleBlur, handleChange } = hooks({ onMount });
</script>

<div class="search" class:focused={$focused}>
	<div class="control">
		<input
			type="search"
			value={$value}
			placeholder="Search for trees"
			onfocus={handleFocus}
			onblur={handleBlur}
			onchange={handleChange}
		/>
		<button onclick={handleReset} class="reset">×</button>
	</div>

	<div class="hints">
		<div>
			<a href={routes.searchQuery('added:604800')}>Added</a> or
			<a href={routes.searchQuery('updated:604800')}>updated</a> last week.
		</div>
		<div><a href={routes.searchQuery('added:me')}>Added</a> by me.</div>
		<div>
			Without <a href={routes.searchQuery('no:height')}>height</a>,
			<a href={routes.searchQuery('no:diameter')}>diameter</a>,
			<a href={routes.searchQuery('no:circumference')}>grith</a>,
			<a href={routes.searchQuery('noimage')}>images</a>,
			<a href={routes.searchQuery('unknown')}>species</a>
			or just <a href={routes.searchQuery('incomplete')}>incomplete</a>.
		</div>
		<div class="advanced"><a href={routes.search()}>Advanced search</a></div>
	</div>
</div>

<style>
	.search {
		position: absolute;
		top: 10px;
		left: 50%;
		transform: translateX(-50%);
		z-index: 1000;
		font-size: 14px;

		border: 2px solid rgba(0, 0, 0, 0.2);
		border-radius: 4px;
		background-color: #fff;

		min-width: 300px;

		.control {
			display: flex;
			flex-direction: row;

			height: 30px;
			line-height: 30px;

			/**
			 * Fix colors according to other map controls.
			 * They are always black on white, regardless of the theme.
			 **/
			background-color: #fff;
			color: #000;

			input {
				flex-grow: 1;
				flex-shrink: 1;
				padding: 0 10px;
				border: none;
				outline: none;
				width: 100%;
			}

			button {
				flex-grow: 0;
				flex-shrink: 0;
				flex-basis: 30px;
				border: none;
				cursor: pointer;
				background-color: transparent;
			}

			input,
			button {
				background-color: inherit;
				color: inherit;
			}
		}

		.hints {
			display: none;
		}

		&.focused .hints {
			margin: 0 10px 10px;
			padding: 5px 0 0;

			color: #000;
			border-top: 1px solid rgba(0, 0, 0, 0.1);

			display: flex;
			flex-direction: column;
			gap: 10px;

			.advanced {
				border-top: 1px solid rgba(0, 0, 0, 0.1);
				padding-top: 5px;
				margin-top: 5px;
			}
		}

		button.reset {
			font-size: 20px;
		}
	}

	/**
	 * Some hot fixes for mobile devices.
	 **/
	@media (max-width: 600px) {
		.search {
			position: absolute;
			top: 10px;
			left: 10px;
			right: 10px;
			min-width: auto;

			transform: none;
		}

		/**
		 * The search bar takes whole device width.
		 * We need to move other controls down a bit.
		 **/
		:global(.leaflet-container.has-search .leaflet-right.leaflet-top) {
			top: 40px;
		}

		:global(.leaflet-container.has-search .leaflet-left.leaflet-top) {
			top: 40px;
		}
	}
</style>
