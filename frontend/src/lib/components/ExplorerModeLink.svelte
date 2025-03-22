<script lang="ts">
	import { locale } from '$lib/locale';
	import { modeStore } from '$lib/stores/modeStore';
	import { routes, goto } from '$lib/routes';
	import { ModeEnum } from '$lib/enums';

	const handleClick = (e: Event) => {
		e.preventDefault();

		modeStore.set(ModeEnum.Explorer);
		console.log('[mode] Entered Explorer Mode');

		goto(routes.map());
	};
</script>

<a href={routes.map()} onclick={handleClick}>{locale.enterExplorerMode()}</a>
