<script lang="ts">
	import SidebarSearchInput from '$lib/components/search/SidebarSearchInput.svelte';
	import CloseIcon from '$lib/icons/CloseIcon.svelte';
	import UserIcon from '$lib/icons/UserIcon.svelte';
	import { locale } from '$lib/locale';
	import { isSidebarVisible } from '$lib/stores/mobileSidebarStore';
	import { ToolsSidebarLogic } from './ToolsSidebar.svelte.ts';

	const componentState = new ToolsSidebarLogic();
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<div
	class="overlay"
	class:visible={!!$isSidebarVisible}
	onclick={componentState.handleOverlayClick}
>
	<div class="canvas">
		<div class="mobile-header">
			<button class="close-btn" onclick={componentState.close}>
				<CloseIcon />
			</button>
		</div>

		<div class="search">
			<SidebarSearchInput
				value={componentState.query}
				onInput={componentState.handleInput}
				onSearch={componentState.handleSearch}
				onClose={componentState.handleClose}
			/>
		</div>

		<h2>{locale.toolsHeading()}</h2>

		<div class="grid">
			{#each componentState.items as item}
				<a href={item.href} class:active={item.active} onclick={componentState.close}>
					<span class="icon"><UserIcon /></span>
					<span class="label">{item.name}</span>
				</a>
			{/each}
		</div>
	</div>
</div>

<style>
	.canvas {
		height: 100dvh;
		width: var(--sidebar-width-desktop);
		position: fixed;
		top: 0;
		left: 0;
		box-sizing: border-box;

		background-color: var(--map-menu-background);

		display: flex;
		flex-direction: column;
		gap: calc(var(--gap) * 2);

		padding: var(--gap);
		padding-bottom: calc(2 * var(--gap));
		border-right: 1px solid var(--sep-color);

		text-align: left;
		font-size: 18px;

		color: var(--text-color);

		h2 {
			font-size: 1.25rem;
			font-weight: 400;
			margin: 0;
			opacity: 0.5;
		}

		.mobile-header {
			display: none;
		}

		.search {
			display: none;
		}

		.grid {
			display: grid;
			grid-template-columns: repeat(4, 1fr);
			gap: var(--gap);
			align-content: start;

			a {
				display: flex;
				flex-direction: column;
				align-items: center;
				gap: calc(var(--gap) / 2);

				color: inherit;
				text-decoration: none;
				text-align: center;
				padding: var(--gap) calc(var(--gap) / 2);
				border-radius: 8px;
				cursor: pointer;

				&:hover,
				&.active {
					background-color: var(--sep-color);
				}
			}

			.icon {
				flex: 0 0 auto;
				height: 32px;
				width: 32px;
				position: relative;
			}

			.label {
				font-size: 14px;
				line-height: 1.2;
			}
		}
	}

	/** Hide by default on small screens. **/
	@media screen and (max-width: 1023px) {
		.overlay {
			animation: fadeIn 0.2s ease-in-out;
			display: none;
			position: fixed;
			top: 0;
			left: 0;
			width: 100vw;
			height: 100dvh;
			background-color: rgba(0, 0, 0, 0.5);
			backdrop-filter: blur(2px);

			&.visible {
				display: block;
			}
		}

		.canvas {
			width: 100vw;
			height: auto;
			top: auto;
			bottom: 0;
			border-right: none;
			border-radius: 16px 16px 0 0;
			padding: var(--gap);
			padding-bottom: calc(2 * var(--gap));
			animation: slideUp 0.2s ease-out;

			.mobile-header {
				position: absolute;
				top: var(--gap);
				right: var(--gap);
				z-index: 1;
				display: block;

				.close-btn {
					background-color: transparent;
					border: none;
					color: light-dark(black, white);
					width: 30px;
					height: 30px;
					padding: 0 4px;
					cursor: pointer;
					opacity: 0.5;
				}
			}
		}
	}

	@keyframes slideUp {
		from {
			transform: translateY(100%);
		}
		to {
			transform: translateY(0);
		}
	}

	/** Must be below the preview on desktop, above on mobile. **/
	.overlay {
		z-index: var(--z-learn-result);
	}

	@media screen and (min-width: 1024px) {
		.canvas .search {
			display: block;
			padding: 0;
		}

		.overlay {
			z-index: 1;
		}
	}
</style>
