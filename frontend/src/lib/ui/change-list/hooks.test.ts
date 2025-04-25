import type { IChange } from '$lib/types';
import { addUsers } from '$lib/stores/userStore';
import { describe, expect, it } from 'vitest';
import { format } from './hooks';

describe('change-list/hooks', () => {
	it('should format correctly', () => {
		const changes = [
			{
				id: 'change1',
				tree_id: 'tree1',
				name: 'height',
				value: '1',
				added_at: 0,
				added_by: 'user1'
			}
		] as IChange[];

		addUsers([
			{
				id: 'user1',
				name: 'John Doe',
				picture: 'https://example.com/johndoe.jpg'
			}
		]);

		const formatted = format(changes);

		expect(formatted).toStrictEqual([
			{
				header: '01.01.1970, John Doe:',
				body: 'height → 1'
			}
		]);
	});

	it('unknown user', () => {
		const changes = [
			{
				id: 'change1',
				tree_id: 'tree1',
				name: 'height',
				value: '1',
				added_at: 0,
				added_by: '0'
			}
		] as IChange[];

		const formatted = format(changes);

		expect(formatted).toStrictEqual([
			{
				header: '01.01.1970, (unknown user):',
				body: 'height → 1'
			}
		]);
	});
});
