import { cleanup, render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { afterEach, describe, expect, test, vi } from 'vitest';
import CanopyInput from './CanopyInput.svelte';

describe('CanopyInput', async () => {
	afterEach(cleanup);

	test('does not show the divider suggestion by default', async () => {
		const user = userEvent.setup();

		render(CanopyInput, {
			value: null,
			onChange: () => {}
		});

		const input = screen.getByRole('spinbutton');
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

		const input = screen.getByRole('spinbutton');
		await user.type(input, '4');

		const suggestion = screen.getByRole('button', { name: '2.0' });
		await user.click(suggestion);

		expect(onChange).toHaveBeenCalledWith(2);
	});
});
