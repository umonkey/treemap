<script lang="ts">
	import { TrashIcon, CreateIcon } from '$lib/icons';
	import '$lib/styles/colors.css';
	import '$lib/styles/animations.css';

	const { visible, onMakeThumbnail, onDelete, onClose } = $props<{
		visible: boolean;
		onMakeThumbnail: () => void;
		onDelete: () => void;
		onClose: () => void;
	}>();
</script>

{#if visible}
	<!-- svelte-ignore a11y_no_static_element_interactions -->
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<div class="canvas" onclick={onClose}>
		<div class="menu">
			<ul>
				<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
				<li onclick={onMakeThumbnail}>
					<CreateIcon />
					<span>Make thumbnail</span>
				</li>
				<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
				<li onclick={onDelete}>
					<TrashIcon />
					<span>Delete</span>
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
				cursor: pointer;
				align-items: center;

				&:hover {
					background-color: rgba(0, 0, 0, 0.05);
				}
			}

			span {
				display: block;
				line-height: 20px;
			}

			:global(li > svg) {
				width: 20px;
				height: 20px;
			}
		}
	}
</style>
