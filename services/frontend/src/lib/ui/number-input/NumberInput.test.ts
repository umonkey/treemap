import { cleanup, render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { afterEach, describe, expect, test, vi } from 'vitest';
import NumberInput from './NumberInput.svelte';

describe('NumberInput', () => {
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

		const input = screen.getByRole('textbox');
		await user.clear(input);
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

		const input = screen.getByRole('textbox');
		await user.clear(input);
		await user.type(input, 'foobar');
		await user.tab();

		expect(value).toBe(123);
	});

	test('accepts a fractional value typed with a dot', async () => {
		const user = userEvent.setup();
		const onChange = vi.fn();

		render(NumberInput, {
			value: null,
			onChange
		});

		const input = screen.getByRole('textbox') as HTMLInputElement;
		await user.type(input, '4.5');

		expect(onChange).toHaveBeenLastCalledWith(4.5);
		expect(input.value).toBe('4.5');
	});

	test('accepts a fractional value typed with a comma', async () => {
		const user = userEvent.setup();
		const onChange = vi.fn();

		render(NumberInput, {
			value: null,
			onChange
		});

		const input = screen.getByRole('textbox') as HTMLInputElement;
		await user.type(input, '4,5');

		expect(onChange).toHaveBeenLastCalledWith(4.5);
		expect(input.value).toBe('4,5');
	});

	test('does not emit a value for invalid text', async () => {
		const user = userEvent.setup();
		const onChange = vi.fn();

		render(NumberInput, {
			value: null,
			onChange
		});

		const input = screen.getByRole('textbox');
		await user.type(input, 'abc');

		expect(onChange).not.toHaveBeenCalled();
	});

	test('reflects an externally updated value', async () => {
		const { rerender } = render(NumberInput, {
			value: 1.5,
			onChange: () => {}
		});

		const input = screen.getByRole('textbox') as HTMLInputElement;
		expect(input.value).toBe('1.5');

		await rerender({ value: 2.25 });

		expect(input.value).toBe('2.25');
	});

	test('accepts a negative value when min allows it', async () => {
		const user = userEvent.setup();
		const onChange = vi.fn();

		render(NumberInput, {
			value: null,
			min: Number.NEGATIVE_INFINITY,
			onChange
		});

		const input = screen.getByRole('textbox') as HTMLInputElement;
		await user.type(input, '-4,5');

		expect(onChange).toHaveBeenLastCalledWith(-4.5);
		expect(input.value).toBe('-4,5');
	});

	test('rejects a negative value with the default minimum', async () => {
		const user = userEvent.setup();
		const onChange = vi.fn();

		render(NumberInput, {
			value: null,
			onChange
		});

		const input = screen.getByRole('textbox');
		await user.type(input, '-4');

		expect(onChange).not.toHaveBeenCalled();
	});
});
