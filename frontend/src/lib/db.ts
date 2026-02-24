import Dexie, { type EntityTable } from 'dexie';

export interface IUpload {
	id?: number;
	tree_id: string;
	status: 'pending' | 'uploading' | 'completed' | 'failed';
	image: Blob;
	created_at: number;
	retry_count: number;
	file_id: string | null;
}

const db = new Dexie('TreeMapUploads') as Dexie & {
	uploads: EntityTable<IUpload, 'id'>;
};

db.version(1).stores({
	uploads: '++id, tree_id, status, created_at'
});

export async function getPendingCount(): Promise<number> {
	return await db.uploads
		.where('status')
		.anyOf('pending', 'failed')
		.filter((u) => u.retry_count < 5)
		.count();
}

export { db };
