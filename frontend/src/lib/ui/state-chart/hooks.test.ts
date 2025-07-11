import { describe, expect, it } from 'vitest';
import { formatChartProps } from './hooks';

describe('charts/state-chart/hooks', () => {
	it('should format correctly', () => {
		const data = [
			{
				state: 'deformed',
				count: 5
			},
			{
				state: 'healthy',
				count: 288
			},
			{
				state: 'sick',
				count: 5
			},
			{
				state: 'stump',
				count: 7
			},
			{
				state: 'unknown',
				count: 2
			}
		];

		const props = formatChartProps(data);

		expect(props).toStrictEqual({
			type: 'pie',
			data: {
				labels: ['deformed (2%)', 'healthy (94%)', 'sick (2%)', 'stump (2%)', 'unknown (1%)'],
				datasets: [
					{
						label: 'Number of trees',
						data: [5, 288, 5, 7, 2]
					}
				]
			},
			options: {
				plugins: {
					legend: {
						position: 'right'
					}
				}
			}
		});
	});
});
