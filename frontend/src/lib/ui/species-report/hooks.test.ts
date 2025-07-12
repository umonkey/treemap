import { describe, expect, it } from 'vitest';
import { formatData } from './hooks';

describe('species-report/hooks.ts', () => {
	it('should format data', () => {
		const input = [
			{
				species: 'oak',
				count: 5
			},
			{
				species: 'elm',
				count: 10
			}
		];

		const res = formatData(input);

		expect(res).toStrictEqual([
			{
				label: 'oak',
				value: 5,
			},
			{
				label: 'elm',
				value: 10,
			}
		]);
	});
});
