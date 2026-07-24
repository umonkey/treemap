export class VideoSyncState {
	videoOffset = $state<number>(0);
	manualOffset = $state<number>(0);

	constructor() {
		// Pure constructor
	}
}
