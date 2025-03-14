<script>
import { locale } from "$lib/locale";
import { formatCentimeters, formatMeters, formatYear } from "$lib/utils/trees";

const { tree } = $props();
</script>

<div class="properties">
	<dl>
		<dt>{locale.propHeight()}</dt>
		<dd>{formatMeters(tree.height)}</dd>
	</dl>
	<dl>
		<dt>{locale.propCanopy()}</dt>
		<dd>{formatMeters(tree.diameter)}</dd>
	</dl>
	<dl>
		<dt>{locale.propTrunk()}</dt>
		<dd>{formatCentimeters(tree.circumference)}</dd>
	</dl>
	<dl>
		<dt>{locale.propState()}</dt>
		<dd>{tree.state}</dd>
	</dl>
	<dl>
		<dt>{locale.propYear()}</dt>
		<dd>{formatYear(tree.year)}</dd>
	</dl>
</div>

<style>
	.properties {
		columns: 2;
		gap: 0;
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
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	dd {
		flex-basis: 50%;
		flex-shrink: 1;
		flex-grow: 1;
		text-align: right;
		margin: 0;
	}

	dd {
		white-space: nowrap;
	}
</style>
