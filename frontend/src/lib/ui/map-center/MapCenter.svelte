<script lang="ts">
	import CROSSHAIR from '$lib/assets/crosshair.svg';
</script>

<img src={CROSSHAIR} class="crosshair" alt="" />

<style>
	.crosshair {
		width: 40px;
		height: 40px;
		position: absolute;
		left: 50%;
		top: 50%;
		z-index: var(--z-crosshair);
		transform: translate(-50%, -50%);
	}
</style>
