<script lang="ts">
	import { locale } from '$lib/locale';
	import { type IHeatMap } from '$lib/types';
	import { HeatMap } from './HeatMap.svelte.ts';

	type Props = {
		data: IHeatMap[];
	};

	const days = [
		locale.shortMonday(),
		'',
		locale.shortWednesday(),
		'',
		locale.shortFriday(),
		'',
		locale.shortSunday()
	];

	const { data }: Props = $props();

	const componentState = new HeatMap();
</script>

<div>
	<div class="heatmap">
		<table>
			<tbody>
				{#each days as day}
					<tr>
						<td class="dow"><span>{day}</span></td>
					</tr>
				{/each}
			</tbody>
		</table>

		<div class="main">
			<table>
				<tbody>
					{#each componentState.formatData(data) as row}
						<tr>
							{#each row as cell}
								<td class="cell" title={cell.title}>
									<div class={`grade${cell.grade}`}></div>
								</td>
							{/each}
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	</div>
</div>

<style>
	.heatmap {
		display: flex;
		flex-direction: row;
		gap: 0.5rem;
		overflow: hidden;

		font-size: 12px;
		line-height: 12px;

		td {
			height: 12px;
		}

		.main {
			height: 114px; /* add 10 px for the scroll bar, make the user able to access the bottom row */
			min-height: 114px;
			width: 100%;
			overflow-x: scroll;
			direction: rtl;
		}
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
</style>
