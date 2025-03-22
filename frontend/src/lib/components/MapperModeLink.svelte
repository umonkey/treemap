<script lang="ts">
	import { locale } from '$lib/locale';
	import { modeStore } from '$lib/stores/modeStore';
	import { routes, goto } from '$lib/routes';

	const handleClick = (e: Event) => {
		e.preventDefault();

		modeStore.set('mapper');
		console.log('[mode] Entered Mapper Mode');

		goto(routes.map());
	};
</script>

<a href={routes.map()} onclick={handleClick}>{locale.enterMapperMode()}</a>
