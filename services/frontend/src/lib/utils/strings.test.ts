import { addUsers } from '$lib/stores/userStore';
import { describe, expect, it } from 'vitest';
import { fileAttribution, formatDate, formatDateTime } from './strings';

describe('formatDate', () => {
	it('should format correctly', () => {
		expect(formatDate(1740304662)).toBe('23.02.2025');
	});
});

describe('formatDateTime', () => {
	it('should format correctly', () => {
		// 1740304662 is 2025-02-23 10:57:42 UTC
		// The test environment might be in a different timezone, so we'll check the format
		const result = formatDateTime(1740304662);
		expect(result).toMatch(/^\d{2}\.\d{2}\.\d{4} \d{2}:\d{2}$/);
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
				picture: 'none.jpg',
				email: 'alice@example.com',
				trees_count: 0,
				comments_count: 0,
				updates_count: 0,
				files_count: 0,
				last_active_at: 0
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
