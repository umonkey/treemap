<script lang="ts">
	/**
	 * This is the pop-up menu shown after clicking the (...) icon on a tree page.
	 *
	 * This component should be placed somewhere on the page, it will be centered
	 * in the container, overlaying that container with a blurry background.
	 */
	import {
		AdjustIcon,
		CameraIcon,
		DiameterIcon,
		EditIcon,
		HeightIcon,
		SkullIcon,
		TrashIcon
	} from '$lib/icons';
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import { menuState } from '$lib/stores/treeMenu';

	const { id } = $props<{
		id: string;
	}>();

	const onCloseMenu = () => {
		menuState.set(false);
	};
</script>

{#if $menuState}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<div class="canvas" onclick={onCloseMenu}>
		<div class="menu">
			<ul>
				<li><EditIcon /> <a href={routes.treeEdit(id)}>{locale.contextEditTree()}</a></li>
				<li>
					<CameraIcon /> <a href={routes.treeUploadPhotos(id)}>{locale.contextUploadPhotos()}</a>
				</li>
				<li>
					<AdjustIcon /> <a href={routes.treeMeasure(id)}>{locale.contextMeasure()}</a>
				</li>
				<li>
					<HeightIcon /> <a href={routes.treeHeight(id)}>{locale.contextHeight()}</a>
				</li>
				<li>
					<DiameterIcon /> <a href={routes.treeDiameter(id)}>{locale.contextDiameter()}</a>
				</li>
				<li>
					<SkullIcon /> <a href={routes.treeDead(id)}>{locale.contextDead()}</a>
				</li>
				<li>
					<TrashIcon /> <a href={routes.treeDelete(id)}>{locale.contextGone()}</a>
				</li>
			</ul>
		</div>
	</div>
{/if}

<style>
	.canvas {
		position: fixed;
		top: 0;
		left: 0;
		height: 100%;
		width: 100%;
		background-color: rgba(0, 0, 0, 0.5);
		backdrop-filter: blur(2px);
		z-index: var(--z-menu);

		.menu {
			background-color: var(--form-background);
			border-radius: 8px;

			position: fixed;
			top: 50%;
			left: 50%;
			transform: translate(-50%, -50%);

			ul {
				list-style-type: none;
				margin: 0;
				padding: var(--gap) 0;
				white-space: nowrap;
			}

			li {
				margin: 0;
				padding: 10px 20px;
				display: flex;
				flex-direction: row;
				gap: var(--gap);
			}

			a {
				display: block;
				line-height: 20px;
			}

			:global(svg) {
				width: 20px;
				height: 20px;
			}
		}
	}
</style>
