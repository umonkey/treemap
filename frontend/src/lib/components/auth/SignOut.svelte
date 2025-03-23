<script>
	import { authStore } from '$lib/stores/authStore';
	import { locale } from '$lib/locale';
	import { routes, goto } from '$lib/routes';

	const onClick = () => {
		authStore.set(undefined);
		goto(routes.home());
	};
</script>

<button class="button" on:click={onClick}>{locale.signOut()}</button>
