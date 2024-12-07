<script>
	const { tree } = $props();
</script>

<div class="properties">
	<dl>
		<dt>Height</dt>
		<dd>{tree.height} m</dd>
	</dl>
	<dl>
		<dt>Canopy</dt>
		<dd>{tree.diameter} m</dd>
	</dl>
	<dl>
		<dt>Trunk</dt>
		<dd>{tree.circumference} m</dd>
	</dl>
	<dl>
		<dt>State</dt>
		<dd>{tree.state}</dd>
	</dl>
	<dl>
		<dt>Year</dt>
		<dd>unknown</dd>
	</dl>
</div>

<style>
	.properties {
		columns: 2;
		gap: 20px;
		margin: var(--gap) 0;

		border-top: 1px solid var(--sep-color);
		padding-top: var(--gap);
		box-sizing: border-box;
	}

	dl {
		display: flex;
		flex-direction: row;
		margin: 0 0 5px;
		padding: 0 var(--gap);
	}

	dt {
		flex-shrink: 1;
		flex-grow: 0;
	}

	dd {
		flex-basis: 50%;
		flex-shring: 1;
		flex-grow: 1;
		text-align: right;
		margin: 0;
	}

	dd {
		white-space: nowrap;
	}
</style>
