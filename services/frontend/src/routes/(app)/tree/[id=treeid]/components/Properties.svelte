<script lang="ts">
	import { locale } from '$lib/locale';
	import { routes } from '$lib/routes';
	import { formatCentimeters, formatMeters, formatState, formatYear } from '$lib/utils/trees';
	import type { ITree } from '$lib/types';

	const { tree } = $props<{
		tree: ITree;
	}>();
</script>

<div class="properties">
	<a class="row" href={routes.treeHeight(tree.id)}>
		<dl>
			<dt>{locale.propHeight()}</dt>
			<dd>{formatMeters(tree.height)}</dd>
		</dl>
	</a>
	<a class="row" href={routes.treeDiameter(tree.id)}>
		<dl>
			<dt>{locale.propCanopy()}</dt>
			<dd>{formatMeters(tree.diameter)}</dd>
		</dl>
	</a>
	<a class="row" href={routes.treeCircumference(tree.id)}>
		<dl>
			<dt>{locale.propTrunk()}</dt>
			<dd>{formatCentimeters(tree.circumference)}</dd>
		</dl>
	</a>
	<dl>
		<dt>{locale.propState()}</dt>
		<dd>{formatState(tree.state)}</dd>
	</dl>
	<dl>
		<dt>{locale.propYear()}</dt>
		<dd>{formatYear(tree.year)}</dd>
	</dl>
</div>

<style>
	.properties {
		columns: 2;
		gap: 1rem;

		box-sizing: border-box;
	}

	.row {
		display: block;
		color: inherit;
		text-decoration: none;
	}

	dl {
		display: flex;
		flex-direction: row;
		margin: 0 0 5px;
		padding: 0;
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
		white-space: nowrap;
		margin: 0;
	}
</style>
