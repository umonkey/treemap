import { goto } from '$app/navigation';
import { vi } from 'vitest';

vi.mock('$app/navigation', async () => {
	return {
		goto: vi.fn()
	};
});

export const mockedGoto = vi.mocked(goto);
