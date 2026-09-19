import { showWarning } from '$lib/errors';
import { locale } from '$lib/locale';

const WINDOW_SIZE = 10;
const SLOW_AVERAGE_SECONDS = 3;
const FAST_AVERAGE_SECONDS = 1;

const recentDurations: number[] = [];
let hasWarned = false;

export function recordRequestDuration(seconds: number): void {
	if (!Number.isFinite(seconds) || seconds < 0) return;

	recentDurations.push(seconds);
	if (recentDurations.length > WINDOW_SIZE) recentDurations.shift();

	const average = recentDurations.reduce((sum, d) => sum + d, 0) / recentDurations.length;

	console.debug(
		`[network] Recorded ${seconds.toFixed(3)}s; average of last ${recentDurations.length}: ${average.toFixed(3)}s`
	);

	if (recentDurations.length < WINDOW_SIZE) return;

	if (!hasWarned && average > SLOW_AVERAGE_SECONDS) {
		hasWarned = true;
		showWarning(locale.toastSlowNetwork());
	} else if (hasWarned && average < FAST_AVERAGE_SECONDS) {
		hasWarned = false;
	}
}

/** Test-only reset. */
export function resetNetworkMonitor(): void {
	recentDurations.length = 0;
	hasWarned = false;
}
