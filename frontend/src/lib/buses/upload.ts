/**
 * This bus is used by the file uploader to tell the components that a file upload was finished.
 */

import mitt from 'mitt';

type UploadBusEvent = {
	success: string; // tree_id
};

export const uploadBus = mitt<UploadBusEvent>();
