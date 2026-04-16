<script lang="ts">
	import CrossHair from '$lib/icons/CrossHair.svelte';
</script>

<div class="center">
	<CrossHair />
</div>

<style>
	.center {
		position: absolute;
		left: 50%;
		top: 50%;
		z-index: 10;
		transform: translate(-50%, -50%);
		width: 50px;
		height: 50px;
	}
</style>
