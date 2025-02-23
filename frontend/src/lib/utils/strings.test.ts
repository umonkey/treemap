import { describe, it, expect } from 'vitest';
import { fileAttribution, formatDate } from './strings';
import { addUsers } from '$lib/stores/userStore';

describe('formatDate', () => {
	it('should format correctly', () => {
		expect(formatDate(1740304662)).toBe('23.02.2025');
	});
});

describe('fileAttribution', () => {
	it('should return an empty string on no data', () => {
		expect(
			fileAttribution({
				id: '1',
				small_id: '2',
				large_id: '3'
			})
		).toBe('');
	});

	it('should return an empty string on no user', () => {
		expect(
			fileAttribution({
				id: '1',
				small_id: '2',
				large_id: '3',
				added_at: 0,
				added_by: '12345'
			})
		).toBe('');
	});

	it('should return the correct attribution', () => {
		addUsers([
			{
				id: '12345',
				name: 'Alice',
				picture: 'none.jpg'
			}
		]);

		expect(
			fileAttribution({
				id: '1',
				small_id: '2',
				large_id: '3',
				added_at: 1740304662,
				added_by: '12345'
			})
		).toBe('23.02.2025 by Alice');
	});
});
