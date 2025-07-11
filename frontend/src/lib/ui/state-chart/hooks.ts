type Item = {
	state: string;
	count: number;
};

const formatLabel = (state: string, count: number, total: number) => {
	const pc = Math.round((count / total) * 100);
	return `${state} (${pc}%)`;
};

export const formatChartProps = (items: Item[]) => {
	const total = items.reduce((sum, item) => sum + item.count, 0);

	const labels = items.map((item) => formatLabel(item.state, item.count, total));
	const values = items.map((item) => item.count);

	return {
		type: 'pie',

		data: {
			labels: labels,

			datasets: [
				{
					label: 'Number of trees',
					data: values
				}
			]
		},

		options: {
			plugins: {
				legend: {
					position: 'right'
				}
			},
			responsive: true,
			maintainAspectRatio: true,
		}
	};
};
