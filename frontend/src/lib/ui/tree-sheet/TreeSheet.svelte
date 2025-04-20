<script lang="ts">
	import type { ITree } from '$lib/types';
	import { formatState } from '$lib/utils/trees';
	import { locale } from '$lib/locale';

	const { tree } = $props<{
		tree: ITree;
	}>();
</script>

<table class="tree-sheet">
	<tbody>
		<tr>
			<th>Id:</th>
			<td>{tree.id}</td>
		</tr>
		<tr>
			<th>{locale.speciesLabel()}:</th>
			<td>{tree.species}</td>
		</tr>
		<tr>
			<th>{locale.stateLabel()}:</th>
			<td>{formatState(tree.state)}</td>
		</tr>
		<tr>
			<th>{locale.addressLabel()}:</th>
			<td>{tree.address ?? locale.addressUnknown()}</td>
		</tr>
	</tbody>
</table>

<style>
	.tree-sheet {
		border-spacing: 0;
		margin: var(--gap) 0;

		th,
		td {
			text-align: left;
			padding: 0 var(--gap) var(--gap) 0;
		}
	}
</style>
