<script lang="ts">
	import { Buttons, Button, Header, NarrowPage } from '$lib/ui';
	import { ModeEnum } from '$lib/enums';
	import { goto, routes } from '$lib/routes';
	import { modeStore } from '$lib/stores/modeStore';

	const handleClick = () => {
		modeStore.set(ModeEnum.Mapper);
		goto(routes.map());
	};
</script>

<svelte:head>
	<title>Mapper mode</title>
</svelte:head>

<Header title="Mapper Mode" />

<NarrowPage>
	<p>You are going to enter the mapper mode.</p>
	<p>This mode lets you add new trees and update data for existing trees.</p>
	<p>
		Refer to <a href="https://myga.am/mapping.html">this page</a> if you need help understanding how
		this works.
	</p>
	<p>Please keep in mind that all changes are logged.</p>

	<Buttons>
		<Button onClick={handleClick}>Continue</Button>
		<Button link="https://myga.am/mapping.html" type="cancel">Help</Button>
	</Buttons>
</NarrowPage>
