import { describe, expect, it } from 'vitest';
import { formatChartProps } from './hooks';

describe('bar-chart/hooks.ts', () => {
	it('should format props', () => {
		const input = [
			{
				label: 'first',
				value: 5
			},
			{
				label: 'second',
				value: 10
			},
		];

		const res = formatChartProps(input);

		expect(res).toStrictEqual({
			type: 'bar',

			data: {
				labels: ['first (33%)', 'second (67%)'],

				datasets: [{
					data: [5, 10],
				}],
			},

			options: {
				plugins: {
					legend: {
						display: false,
					},
				},
				responsive: true,
				maintainAspectRatio: false,
			},
		});
	});
});
