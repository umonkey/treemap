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
		type: 'bar',

		data: {
			labels: labels,

			datasets: [
				{
					data: values
				}
			]
		},

		options: {
			plugins: {
				legend: {
					display: false
				}
			},

			scales: {
				x: {
					ticks: {
						minRotation: 90,
						maxRotation: 90
					}
				}
			},

			responsive: true,
			maintainAspectRatio: false
		}
	};
};
