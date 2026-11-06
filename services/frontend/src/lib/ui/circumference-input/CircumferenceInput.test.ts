import { cleanup, render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { afterEach, describe, expect, test, vi } from 'vitest';
import CircumferenceInput from './CircumferenceInput.svelte';

describe('CircumferenceInput', () => {
	afterEach(cleanup);

	test('accepts a fractional value typed with a comma', async () => {
		const user = userEvent.setup();
		const onChange = vi.fn();

		render(CircumferenceInput, {
			value: null,
			onChange
		});

		const input = screen.getByRole('textbox') as HTMLInputElement;
		await user.type(input, '144,5');

		expect(onChange.mock.lastCall?.[0]).toBeCloseTo(1.445);
		expect(input.value).toBe('144,5');
	});

	test('keeps the typed comma when the parent echoes the value', async () => {
		const user = userEvent.setup();

		const { rerender } = render(CircumferenceInput, {
			value: null,
			onChange: () => {}
		});

		const input = screen.getByRole('textbox') as HTMLInputElement;
		await user.type(input, '144,5');

		await rerender({ value: 1.445 });

		expect(input.value).toBe('144,5');
	});

	test('renders an initial value in centimetres', () => {
		render(CircumferenceInput, {
			value: 1.445,
			onChange: () => {}
		});

		const input = screen.getByRole('textbox') as HTMLInputElement;
		expect(input.value).toBe('144.5');
	});
});
