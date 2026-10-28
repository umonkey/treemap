import { showError, showInfo } from '$lib/errors';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { handleShareTree } from './shareTree';

vi.mock('$lib/routes', () => ({
	routes: {
		treeDetails: (id: string) => `/tree/${id}`
	}
}));

vi.mock('$lib/errors', () => ({
	showError: vi.fn(),
	showInfo: vi.fn()
}));

const mockedShowError = vi.mocked(showError);
const mockedShowInfo = vi.mocked(showInfo);

const share = vi.fn<(data: ShareData) => Promise<void>>();

const setShare = (value: typeof share | undefined) => {
	Object.defineProperty(navigator, 'share', {
		value,
		configurable: true
	});
};

describe('handleShareTree', () => {
	let consoleError: ReturnType<typeof vi.spyOn>;

	beforeEach(() => {
		mockedShowError.mockReset();
		mockedShowInfo.mockReset();
		share.mockReset();
		share.mockResolvedValue(undefined);
		setShare(share);
		consoleError = vi.spyOn(console, 'error').mockImplementation(() => {});
	});

	afterEach(() => {
		consoleError.mockRestore();
		setShare(undefined);
	});

	it('shares the tree details URL', async () => {
		await handleShareTree('451036810299183118');

		expect(share).toHaveBeenCalledWith({
			title: document.title,
			url: '/tree/451036810299183118'
		});
		expect(consoleError).not.toHaveBeenCalled();
		expect(mockedShowError).not.toHaveBeenCalled();
	});

	it('stays silent when the user cancels the share sheet', async () => {
		share.mockRejectedValue(new DOMException('Share canceled', 'AbortError'));

		await handleShareTree('451036810299183118');

		expect(consoleError).not.toHaveBeenCalled();
		expect(mockedShowError).not.toHaveBeenCalled();
	});

	it('stays silent when a plain error reports an abort', async () => {
		share.mockRejectedValue(Object.assign(new Error('Share canceled'), { name: 'AbortError' }));

		await handleShareTree('451036810299183118');

		expect(consoleError).not.toHaveBeenCalled();
		expect(mockedShowError).not.toHaveBeenCalled();
	});

	it('reports other failures', async () => {
		const error = new DOMException('Permission denied', 'NotAllowedError');
		share.mockRejectedValue(error);

		await handleShareTree('451036810299183118');

		expect(consoleError).toHaveBeenCalledWith('Error sharing a tree.', error);
		expect(mockedShowError).toHaveBeenCalledWith(`Error sharing this page: ${error}`);
	});

	it('informs when the browser cannot share', async () => {
		setShare(undefined);

		await handleShareTree('451036810299183118');

		expect(mockedShowInfo).toHaveBeenCalled();
		expect(consoleError).not.toHaveBeenCalled();
		expect(mockedShowError).not.toHaveBeenCalled();
	});
});
