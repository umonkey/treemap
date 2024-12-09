<script>
	import DotsIcon from '$lib/icons/DotsIcon.svelte';

	const { title, address } = $props();
</script>

<div class="title">
	<h1><strong>{title}</strong>{#if address} &middot; {address}{/if}</h1>
	<div class="icon">
		<DotsIcon />
	</div>
</div>

<style>
	.title {
		display: flex;
		justify-content: space-between;
		align-items: center;
		height: 50px;
		padding: 0 var(--gap);

		h1 {
			font-size: 1rem;
			font-weight: 400;
			line-height: 50px;
			margin: 0;
		}

		.icon {
			flex-basis: 25px;
			color: var(--icon-color-secondary);
		}
	}
</style>
