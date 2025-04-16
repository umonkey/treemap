<script lang="ts">
import { ModeEnum } from "$lib/enums";
import { locale } from "$lib/locale";
import { goto, routes } from "$lib/routes";
import { modeStore } from "$lib/stores/modeStore";

const handleClick = (e: Event) => {
	e.preventDefault();

	modeStore.set(ModeEnum.Mapper);
	console.log("[mode] Entered Mapper Mode");

	goto(routes.map());
};
</script>

<a href={routes.map()} onclick={handleClick}>{locale.enterMapperMode()}</a>
