import Dexie, { type EntityTable } from 'dexie';

/**
 * Last completed step of an upload. Records written by older app versions have
 * no stage and must be treated as 'new'.
 */
export type UploadStage = 'new' | 'uploaded' | 'finished';

export interface IUpload {
	id?: number;
	tree_id: string;
	status: 'pending' | 'uploading' | 'completed' | 'failed';
	image: Blob;
	thumbnail?: Blob;
	created_at: number;
	retry_count: number;
	file_id: string | null;
	progress?: number;
	stage: UploadStage;
	permanently_failed?: boolean;
}

const db = new Dexie('TreeMapUploads') as Dexie & {
	uploads: EntityTable<IUpload, 'id'>;
};

db.version(2).stores({
	uploads: '++id, tree_id, status, created_at'
});

export async function getPendingCount(): Promise<number> {
	return await db.uploads.count();
}

export { db };
