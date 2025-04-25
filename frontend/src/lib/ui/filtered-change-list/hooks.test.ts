import type { IChange } from '$lib/types';
import { addUsers } from '$lib/stores/userStore';
import { describe, expect, it } from 'vitest';
import { filter, format } from './hooks';

describe('FilteredChangeList hooks', () => {
	it('should filter correctly', () => {
		const input = [
			{
				id: 'change1',
				tree_id: 'tree1',
				name: 'height',
				value: '1',
				added_at: 0,
				added_by: 'user1'
			},
			{
				id: 'change2',
				tree_id: 'tree1',
				name: 'diameter',
				value: '1',
				added_at: 0,
				added_by: 'user1'
			}
		] as IChange[];

		const filtered = filter(input, 'height');
		expect(filtered).toHaveLength(1);
		expect(filtered[0].name).toBe('height');
	});

	it('should format correctly', () => {
		addUsers([
			{
				id: 'user1',
				name: 'John Doe',
				picture: 'https://example.com/johndoe.jpg'
			}
		]);

		const input = [
			{
				id: 'change1',
				tree_id: 'tree1',
				name: 'height',
				value: '1',
				added_at: 1745570403,
				added_by: 'user1'
			}
		] as IChange[];

		const res = format(input, 'height');

		expect(res).toStrictEqual([
			{
				date: '25.04.2025',
				value: '1',
				author: 'John Doe'
			}
		]);
	});
});
