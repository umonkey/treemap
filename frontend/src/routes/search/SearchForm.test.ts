import userEvent from '@testing-library/user-event';
import { cleanup, render, screen } from '@testing-library/svelte';
import { afterEach, describe, expect, test } from 'vitest';
import SearchForm from './SearchForm.svelte';

describe('SearchForm', async () => {
	afterEach(cleanup);

	test('shows search link', async () => {
		const user = userEvent.setup();

		render(SearchForm);

		const input = screen.getByTestId('search-input');
		await user.type(input, 'hello world');

		const link = screen.getByTestId('search-link');
		expect(link.textContent).toBe('Search the map for "hello world"');
	});
});
