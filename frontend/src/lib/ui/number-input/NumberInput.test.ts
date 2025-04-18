import { cleanup, render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { afterEach, describe, expect, test } from 'vitest';
import NumberInput from './NumberInput.svelte';

describe('NumberInput', async () => {
	afterEach(cleanup);

	test('handle number input', async () => {
		const user = userEvent.setup();

		let value = 0;

		render(NumberInput, {
			value,
			onChange: (v: number) => {
				value = v;
			}
		});

		const input = screen.getByRole('spinbutton');
		await user.type(input, '123');
		await user.tab();

		expect(value).toBe(123);
	});

	test('handle bad input', async () => {
		const user = userEvent.setup();

		let value = 123;

		render(NumberInput, {
			value,
			onChange: (v: number) => {
				value = v;
			}
		});

		const input = screen.getByRole('spinbutton');
		await user.type(input, 'foobar');
		await user.tab();

		expect(value).toBe(123);
	});
});
