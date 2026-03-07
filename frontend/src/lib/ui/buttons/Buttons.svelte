<script lang="ts">
	const { children, sticky = false } = $props();
</script>

<div class="buttons" class:sticky={!!sticky}>
	{@render children()}
</div>

<style>
	.buttons {
		gap: var(--gap);
		display: flex;
		flex-direction: row;
	}

	@media screen and (max-width: 600px) {
		.sticky {
			position: fixed;
			bottom: 50px;
			left: 0;
			right: 0;
			height: 34px;
			padding: var(--gap);
			background-color: var(--background-color);
		}
	}
</style>
