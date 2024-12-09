<script>
	import BackIcon from '$lib/icons/BackIcon.svelte';

	const onBack = () => {
		history.back();
	};
</script>

<div class="header">
	<button class="icon" on:click={onBack}><BackIcon /></button>
	<div>Tree</div>
	<div class="icon"></div>
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

		button {
			background: transparent;
			border: none;
			color: var(--text-color);
			display: block;
			cursor: pointer;
		}

		.icon {
			flex-basis: 30px;
			flex-grow: 0;
			flex-shrink: 0;
			height: 30px;
			align-content: center;
		}
	}
</style>
