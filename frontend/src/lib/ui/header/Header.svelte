<script lang="ts">
	import { mobileSidebarStore } from '$lib/stores/mobileSidebarStore';
	import { BackIcon, BarsIcon } from '$lib/icons';

	const { title, back = true } = $props<{
		title: string;
		back?: boolean;
	}>();

	const onBack = () => {
		history.back();
	};

	const onBars = () => {
		mobileSidebarStore.update((state) => !state);
	};
</script>

<div class="header">
	<div class="left">
		{#if back}
			<button type="button" class="icon" onclick={onBack}><BackIcon /></button>
		{/if}
	</div>
	<div class="title">{title}</div>
	<div class="right">
		<button type="button" class="icon bars" onclick={onBars}><BarsIcon /></button>
	</div>
</div>

<style>
	.header {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0 var(--gap);
		border-bottom: 1px solid var(--sep-color);
		font-weight: 800;
		height: 40px;

		.left,
		.right {
			flex-basis: 30px;
			flex-grow: 0;
			flex-shrink: 0;
		}

		.title {
			flex-grow: 1;
			text-align: center;
		}
	}

	button {
		background: transparent;
		border: none;
		color: var(--text-color);
		display: block;
		cursor: pointer;
	}

	button.icon {
		height: 30px;
		align-content: center;
	}

	button.bars {
		display: block;
	}

	/* Hide burger menu on desktop */
	@media (min-width: 1024px) {
		button.bars {
			display: none;
		}
	}
</style>
