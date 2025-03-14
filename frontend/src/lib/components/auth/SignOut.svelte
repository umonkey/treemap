<script>
import { locale } from "$lib/locale";
import { authStore } from "$lib/stores/authStore";

const onClick = () => {
	authStore.set(undefined);
};
</script>

<button class="button" on:click={onClick}>{locale.signOut()}</button>
