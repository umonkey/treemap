<script>
	import { authStore } from '$lib/stores/authStore';
	import { locale } from '$lib/locale';

	const onClick = () => {
		authStore.set(undefined);
	};
</script>

<button class="button" on:click={onClick}>{locale.signOut()}</button>
