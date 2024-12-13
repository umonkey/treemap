<script>
	import { authState } from '$lib/stores/auth';

	const onClick = () => {
		authState.set(undefined);
	};
</script>

<button class="btn" on:click={onClick}>Sign out</button>

<style>
	.btn {
		padding: 10px 20px;
		cursor: pointer;
	}
</style>
