<script>
	import { authState } from '$lib/stores/auth';

	const onClick = () => {
		authState.set(undefined);
	};
</script>

<button class="button" on:click={onClick}>Sign out</button>
