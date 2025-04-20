<script lang="ts">
	import { Buttons, Button, Header } from '$lib/ui';
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

<div class="mapper-mode padded">
	<p>You are going to enter the mapper mode.</p>
	<p>This mode lets you add new trees and update data for existing trees.</p>
	<p>Please keep in mind that all changes are logged.</p>

	<Buttons>
		<Button label="Continue" onClick={handleClick} />
	</Buttons>
</div>
