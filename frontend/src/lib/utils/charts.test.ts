import { describe, expect, it } from 'vitest';
import { normalizeNumericBarChart } from './charts';

describe('charts.ts', () => {
	it('should normalize numeric bar chart', () => {
		const input = [
			{
				value: 2,
				count: 5
			},
			{
				value: 4,
				count: 10
			},
			{
				value: 5,
				count: 15
			}
		];

		const res = normalizeNumericBarChart(input);

		expect(res).toStrictEqual([
			{
				value: 2,
				count: 5
			},
			{
				value: 3,
				count: 0
			},
			{
				value: 4,
				count: 10
			},
			{
				value: 5,
				count: 15
			}
		]);
	});
});
