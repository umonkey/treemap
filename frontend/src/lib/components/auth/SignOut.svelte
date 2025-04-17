<script>
import { locale } from "$lib/locale";
import { goto, routes } from "$lib/routes";
import { authStore } from "$lib/stores/authStore";

const onClick = () => {
	authStore.set(undefined);
	goto(routes.home());
};
</script>

<button class="button" on:click={onClick}>{locale.signOut()}</button>
