<script lang="ts">
	import { Form, Buttons, Button, StreetInput, StreetReport } from '$lib/ui';
	import { hooks } from './hooks';

	const { report, handleStreetChange, handleSubmit } = hooks();
</script>

<div class="report">
	<Form>
		<StreetInput onChange={handleStreetChange} />

		<Buttons>
			<Button onClick={handleSubmit}>Generate</Button>
		</Buttons>
	</Form>
</div>

{#if $report}
	<StreetReport data={$report} />
{/if}
