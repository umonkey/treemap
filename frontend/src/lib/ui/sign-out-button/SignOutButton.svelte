<script lang="ts">
	import { locale } from '$lib/locale';
	import { goto, routes } from '$lib/routes';
	import { authStore } from '$lib/stores/authStore';
	import { Button } from '$lib/ui';

	const onClick = () => {
		authStore.set(undefined);
		goto(routes.home());
	};
</script>

<Button {onClick} label={locale.signOut()} />
