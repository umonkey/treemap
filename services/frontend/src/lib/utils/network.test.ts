import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { showWarning } from '$lib/errors';
import { recordRequestDuration, resetNetworkMonitor } from './network';

vi.mock('$lib/errors', () => ({
	showWarning: vi.fn()
}));

describe('network monitor', () => {
	beforeEach(() => {
		resetNetworkMonitor();
		vi.clearAllMocks();
	});

	afterEach(() => {
		resetNetworkMonitor();
		vi.resetAllMocks();
	});

	it('does not warn before a full window of measurements', () => {
		for (let i = 0; i < 9; i++) {
			recordRequestDuration(4);
		}

		expect(showWarning).not.toHaveBeenCalled();
	});

	it('warns exactly once when the 10th slow measurement completes the window', () => {
		for (let i = 0; i < 10; i++) {
			recordRequestDuration(4);
		}

		expect(vi.mocked(showWarning)).toHaveBeenCalledTimes(1);
	});

	it('does not warn again while the latch is set', () => {
		for (let i = 0; i < 20; i++) {
			recordRequestDuration(4);
		}

		expect(vi.mocked(showWarning)).toHaveBeenCalledTimes(1);
	});

	it('keeps only the last 10 measurements', () => {
		for (let i = 0; i < 10; i++) {
			recordRequestDuration(4);
		}

		expect(vi.mocked(showWarning)).toHaveBeenCalledTimes(1);
		vi.mocked(showWarning).mockClear();

		// Ten fast samples evict the slow ones from the ring buffer.
		for (let i = 0; i < 10; i++) {
			recordRequestDuration(0.1);
		}

		// A single slow sample now leaves the average below the threshold,
		// proving the previous slow samples were evicted.
		recordRequestDuration(4);

		expect(vi.mocked(showWarning)).not.toHaveBeenCalled();
	});

	it('resets the latch once the average drops below one second', () => {
		for (let i = 0; i < 10; i++) {
			recordRequestDuration(4);
		}

		expect(vi.mocked(showWarning)).toHaveBeenCalledTimes(1);
		vi.mocked(showWarning).mockClear();

		for (let i = 0; i < 10; i++) {
			recordRequestDuration(0.1);
		}

		for (let i = 0; i < 10; i++) {
			recordRequestDuration(4);
		}

		expect(vi.mocked(showWarning)).toHaveBeenCalledTimes(1);
	});

	it('ignores non-finite and negative durations', () => {
		recordRequestDuration(NaN);
		recordRequestDuration(Infinity);
		recordRequestDuration(-Infinity);
		recordRequestDuration(-5);

		expect(vi.mocked(showWarning)).not.toHaveBeenCalled();

		for (let i = 0; i < 9; i++) {
			recordRequestDuration(4);
		}

		expect(vi.mocked(showWarning)).not.toHaveBeenCalled();

		recordRequestDuration(4);

		expect(vi.mocked(showWarning)).toHaveBeenCalledTimes(1);
	});
});
