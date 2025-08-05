<script lang="ts">
	import { formatData } from './hooks';
	import { type IHeatMap } from '$lib/types';
	import { locale } from '$lib/locale';

	type Props = {
		title?: string;
		data: IHeatMap;
		docs?: string;
	};

	const days = ['Mon', '', 'Wed', '', 'Fri', '', 'Sun'];

	const { title, data, docs }: Props = $props();
</script>

{#if title}
	<h2>{title}</h2>
{/if}

<div class="heatmap">
	<table>
		<tbody>
			{#each formatData(data) as row, rowIndex}
				<tr>
					<td class="dow"><span>{days[rowIndex]}</span></td>
					{#each row as cell}
						<td class="cell {cell.class}" style="background-color: {cell.color}" title={cell.title}>
							<div class={`grade${cell.grade}`}></div>
						</td>
					{/each}
				</tr>
			{/each}
		</tbody>
	</table>
</div>

{#if docs}
	<div class="docs"><a href={docs} target="_blank">{locale.learnContributions()}</a></div>
{/if}

<style>
	h2 {
		margin-top: 40px;
		font-weight: 400;
		font-size: 20px;
		opacity: 0.75;
	}

	.heatmap {
		height: 110px; /* add 10 px for the scroll bar, make the user able to access the bottom row */
		width: 100%;
		overflow-x: scroll;
	}

	table {
		font-size: 12px;

		td.dow {
			padding-right: 5px;
			height: 10px;
			line-height: 10px;

			span {
				margin-top: -2px;
				opacity: 0.75;
			}
		}

		td.cell > div {
			aspect-ratio: 1;
			width: 10px;
			border-radius: 2px;
		}

		.grade0 {
			background-color: light-dark(#eff2f5, #2a313c);
		}

		.grade1 {
			background-color: light-dark(#aceebb, #1b4721);
		}

		.grade2 {
			background-color: light-dark(#4ac26b, #2b6a30);
		}

		.grade3 {
			background-color: light-dark(#2da44e, #46954a);
		}

		.grade4 {
			background-color: light-dark(#116329, #6bc46d);
		}
	}

	.docs {
		a {
			color: inherit;
			opacity: 0.5;
			font-size: 80%;
		}
	}
</style>
