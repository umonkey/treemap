/**
 * Run a callback when the page regains focus or becomes visible again.
 * Returns a cleanup function that removes the listeners.
 */
export function onPageFocus(callback: () => void): () => void {
	const handleFocus = () => {
		if (document.visibilityState === 'hidden') return;
		callback();
	};
	const handleVisibilityChange = () => handleFocus();
	window.addEventListener('focus', handleFocus);
	document.addEventListener('visibilitychange', handleVisibilityChange);
	return () => {
		window.removeEventListener('focus', handleFocus);
		document.removeEventListener('visibilitychange', handleVisibilityChange);
	};
}
