<script lang="ts">
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import {
		EditIcon,
		MapIcon,
		SkullIcon,
		TrashIcon,
		TreeIcon,
		MeasureHeight,
		MeasureDiameter,
		MeasureGrith
	} from '$lib/icons';
	import '$lib/styles/colors.css';
	import '$lib/styles/animations.css';

	const { id, visible, onClose } = $props<{
		id: string;
		visible: boolean;
		onClose: () => void;
	}>();
</script>

{#if visible}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<div class="canvas" onclick={onClose}>
		<div class="menu">
			<ul>
				<li class="measure">
					<p>Measure:</p>
					<div class="icons">
						<div class="icon">
							<a href={routes.treeHeight(id)}>
								<MeasureHeight />
							</a>
						</div>
						<div class="icon">
							<a href={routes.treeDiameter(id)}>
								<MeasureDiameter />
							</a>
						</div>
						<div class="icon">
							<a href={routes.treeCircumference(id)}>
								<MeasureGrith />
							</a>
						</div>
					</div>
				</li>
				<li class="sep">
					<MapIcon /> <a href={routes.treeMove(id)}>{locale.contextMove()}</a>
				</li>
				<li class="sep">
					<SkullIcon /> <a href={routes.treeDead(id)}>{locale.contextDead()}</a>
				</li>
				<li>
					<TrashIcon /> <a href={routes.treeDelete(id)}>{locale.contextGone()}</a>
				</li>
				<li class="sep">
					<EditIcon /> <a href={routes.treeEdit(id)}>{locale.contextEditTree()}</a>
				</li>
				<li>
					<TreeIcon /> <a href={routes.treeReplace(id)}>{locale.contextReplace()}</a>
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

		animation: fadeIn 0.1s ease-in-out;

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

			:global(li > svg) {
				width: 20px;
				height: 20px;
			}
		}

		li.sep {
			margin-top: 5px;
			padding-top: 10px;
			border-top: solid 1px rgba(0, 0, 0, 0.2);
		}

		li.measure {
			display: flex;
			flex-direction: column;

			a {
				color: inherit;
				display: block;
				aspect-ratio: 1;
			}

			p {
				margin: 0;
				text-align: center;
				opacity: 0.5;
			}

			.icons {
				display: flex;
				flex-direction: row;
				width: 100%;
				gap: 10px;

				.icon {
					flex-grow: 1;
					flex-shrink: 1;

					:global(svg) {
						width: 100%;
						height: 100%;
						aspect-ratio: 1;
					}
				}
			}
		}
	}
</style>
