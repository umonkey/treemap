<script lang="ts">
	import { Buttons, Button } from '$lib/ui';
	import Dialog from '$lib/components/layout/Dialog.svelte';
	import { ModeEnum } from '$lib/enums';
	import { goto, routes } from '$lib/routes';
	import { modeStore } from '$lib/stores/modeStore';
	import MappingOptions from '$lib/components/MappingOptions.svelte';

	const handleClick = () => {
		modeStore.set(ModeEnum.Mapper);
		goto(routes.map());
	};
</script>

<Dialog title="Mapper Mode">
	<p>You are going to enter the mapper mode.</p>
	<p>This mode lets you add new trees and update data for existing trees.</p>
	<p>
		Refer to <a href="https://myga.am/mapping/">this page</a> if you need help understanding how this
		works.
	</p>
	<p>Please keep in mind that all changes are logged.</p>

	<MappingOptions />

	<Buttons>
		<Button onClick={handleClick}>Continue</Button>
		<Button link="https://myga.am/mapping/" type="cancel">Help</Button>
	</Buttons>
</Dialog>
