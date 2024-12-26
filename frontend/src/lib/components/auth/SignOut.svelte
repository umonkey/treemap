<script>
	import { authState } from '$lib/stores/auth';
	import { locale } from '$lib/locale';

	const onClick = () => {
		authState.set(undefined);
	};
</script>

<button class="button" on:click={onClick}>{locale.signOut()}</button>
