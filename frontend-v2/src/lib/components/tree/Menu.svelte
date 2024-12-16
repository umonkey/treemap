<script>
	import { menuState } from '$lib/stores/treeMenu';
	import { routes } from '$lib/routes';

	export let id;

	const onCloseMenu = () => {
		menuState.set(false);
	};
</script>

{#if $menuState}
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<!-- svelte-ignore a11y_click_events_have_key_events -->
	<div class="canvas" on:click={onCloseMenu}>
		<div class="menu">
			<ul>
				<li><a href={routes.treeEdit(id)}>Edit this tree</a></li>
				<li><a href={routes.treeUploadPhotos(id)}>Upload photos</a></li>
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
	}

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
		}

		li {
			margin: 0;
			padding: 0;
		}

		a {
			display: block;
			line-height: 20px;
			padding: 10px 20px;
		}
	}
</style>
