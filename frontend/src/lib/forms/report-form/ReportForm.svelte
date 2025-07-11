<script lang="ts">
	import { Form, Buttons, Button, StreetInput } from '$lib/ui';
	import { hooks } from './hooks';

	const { handleStreetChange, handleSubmit } = hooks();
</script>

<div class="report">
	<Form>
		<StreetInput onChange={handleStreetChange} />

		<Buttons>
			<Button onClick={handleSubmit}>Generate</Button>
		</Buttons>
	</Form>
</div>
