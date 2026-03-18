export type CallbackFn = (fn: () => void) => void;

export class Debouncer {
	timeoutId: ReturnType<typeof setTimeout> | undefined = undefined;
	delay: number = 500;

	public constructor(delay: number = 500) {
		this.delay = delay;
	}

	public run(callback: CallbackFn) {
		clearTimeout(this.timeoutId);
		this.timeoutId = setTimeout(callback, this.delay);
	}
}
