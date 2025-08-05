<script lang="ts">
	import { formatData } from './hooks';
	import { type IHeatMap } from '$lib/types';

	type Props = {
		data: IHeatMap;
	};

	const { data }: Props = $props();
</script>

<div class="heatmap">
	<table>
		<tbody>
			{#each formatData(data) as row, rowIndex}
				<tr>
					{#each row as cell, colIndex}
						<td class="cell {cell.class}" style="background-color: {cell.color}" title={cell.title}>
							<div class={`grade${cell.grade}`}></div>
						</td>
					{/each}
				</tr>
			{/each}
		</tbody>
	</table>
</div>

<style>
	.heatmap {
		height: 100px;
		width: 100%;
		overflow-x: scroll;
	}

	table {
		td.cell > div {
			aspect-ratio: 1;
			width: 10px;
			border-radius: 2px;
		}

		.grade0 {
			background-color: #2a313c;
		}

		.grade1 {
			background-color: #1b4721;
		}

		.grade2 {
			background-color: #2b6a30;
		}

		.grade3 {
			background-color: #46954a;
		}

		.grade4 {
			background-color: #6bc46d;
		}
	}
</style>
