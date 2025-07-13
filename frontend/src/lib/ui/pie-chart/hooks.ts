type Item = {
	label: string;
	value: number;
};

const formatLabel = (state: string, count: number, total: number) => {
	const pc = Math.round((count / total) * 100);
	return `${state} (${pc}%)`;
};

export const formatChartProps = (items: Item[]) => {
	const total = items.reduce((sum, item) => sum + item.value, 0);

	const labels = items.map((item) => formatLabel(item.label, item.value, total));
	const values = items.map((item) => item.value);

	return {
		type: 'pie',

		data: {
			labels: labels,

			datasets: [
				{
					// label: 'Number of trees',
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
			maintainAspectRatio: false
		}
	};
};
