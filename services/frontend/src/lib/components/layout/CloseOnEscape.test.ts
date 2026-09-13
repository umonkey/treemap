import { cleanup, fireEvent, render } from '@testing-library/svelte';
import { afterEach, describe, expect, test, vi } from 'vitest';
import CloseOnEscape from './CloseOnEscape.svelte';

describe('CloseOnEscape', () => {
	afterEach(cleanup);

	test('calls onClose when Escape is pressed', async () => {
		const onClose = vi.fn();

		render(CloseOnEscape, { props: { onClose } });

		await fireEvent.keyDown(window, { key: 'Escape' });

		expect(onClose).toHaveBeenCalledTimes(1);
	});

	test('does not call onClose for other keys', async () => {
		const onClose = vi.fn();

		render(CloseOnEscape, { props: { onClose } });

		await fireEvent.keyDown(window, { key: 'Enter' });

		expect(onClose).not.toHaveBeenCalled();
	});
});
