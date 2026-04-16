export type CallbackFn = (fn: () => void) => void;

export class Debouncer {
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	timeoutId: any = undefined;
	delay = 500;

	public constructor(delay = 500) {
		this.delay = delay;
	}

	public run(callback: CallbackFn) {
		clearTimeout(this.timeoutId);
		this.timeoutId = setTimeout(callback, this.delay);
	}
}
