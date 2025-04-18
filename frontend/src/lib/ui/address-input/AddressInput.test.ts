import { cleanup, render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { afterEach, describe, expect, test } from 'vitest';
import AddressInput from './AddressInput.svelte';

describe('AddressInput', async () => {
	afterEach(cleanup);

	test('handle text', async () => {
		const user = userEvent.setup();

		let value = '';

		render(AddressInput, {
			value,
			onChange: (v: string) => {
				value = v;
			}
		});

		const input = screen.getByRole('textbox');
		await user.type(input, 'test');
		await user.tab();

		expect(value).toBe('test');
	});
});
