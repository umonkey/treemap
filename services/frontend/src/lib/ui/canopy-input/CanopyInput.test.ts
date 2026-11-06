import { cleanup, render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { afterEach, describe, expect, test, vi } from 'vitest';
import CanopyInput from './CanopyInput.svelte';

describe('CanopyInput', () => {
	afterEach(cleanup);

	test('does not show the divider suggestion by default', async () => {
		const user = userEvent.setup();

		render(CanopyInput, {
			value: null,
			onChange: () => {}
		});

		const input = screen.getByRole('textbox') as HTMLInputElement;
		await user.type(input, '4');

		expect(screen.queryByRole('button', { name: '2.0' })).toBeNull();
	});

	test('shows the divider suggestion when enabled', async () => {
		const user = userEvent.setup();
		const onChange = vi.fn();

		render(CanopyInput, {
			value: null,
			showDivider: true,
			onChange
		});

		const input = screen.getByRole('textbox') as HTMLInputElement;
		await user.type(input, '4');

		const suggestion = screen.getByRole('button', { name: '2.0' });
		await user.click(suggestion);

		expect(onChange).toHaveBeenCalledWith(2);
	});

	test('updates the visible input when the divider suggestion is clicked', async () => {
		const user = userEvent.setup();
		const onChange = vi.fn();

		render(CanopyInput, {
			value: null,
			showDivider: true,
			onChange
		});

		const input = screen.getByRole('textbox') as HTMLInputElement;
		await user.type(input, '4');
		await user.click(screen.getByRole('button', { name: '2.0' }));

		expect(input.value).toBe('2');
	});

	test('accepts a fractional value typed with a dot', async () => {
		const user = userEvent.setup();
		const onChange = vi.fn();

		render(CanopyInput, {
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

		render(CanopyInput, {
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

		render(CanopyInput, {
			value: null,
			onChange
		});

		const input = screen.getByRole('textbox') as HTMLInputElement;
		await user.type(input, 'abc');

		expect(onChange).not.toHaveBeenCalled();
	});

	test('reflects an externally updated value', async () => {
		const { rerender } = render(CanopyInput, {
			value: 1.5,
			onChange: () => {}
		});

		const input = screen.getByRole('textbox') as HTMLInputElement;
		expect(input.value).toBe('1.5');

		await rerender({ value: 2.25 });

		expect(input.value).toBe('2.25');
	});
});
