import { cleanup, render, screen } from '@testing-library/svelte';
import userEvent from '@testing-library/user-event';
import { afterEach, describe, expect, test } from 'vitest';
import Button from './Button.svelte';

describe('Button', async () => {
	afterEach(cleanup);

	test('handle click', async () => {
		const user = userEvent.setup();

		let clicked = false;

		render(Button, {
			props: {
				children: () => 'Click Me',
				onClick: () => {
					clicked = true;
				}
			}
		});

		const input = screen.getByRole('button');

		await user.click(input);

		expect(clicked).toBe(true);
	});

	test('handle disabled click', async () => {
		const user = userEvent.setup();

		let clicked = false;

		render(Button, {
			props: {
				disabled: true,
				children: () => 'Click Me',
				onClick: () => {
					clicked = true;
				}
			}
		});

		const input = screen.getByRole('button');
		await user.click(input);

		expect(clicked).toBe(false);
	});
});
