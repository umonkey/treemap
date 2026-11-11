import { locale } from '$lib/locale';
import type { IComment } from '$lib/types';
import { cleanup, render } from '@testing-library/svelte';
import { afterEach, describe, expect, test } from 'vitest';
import Comment from './Comment.svelte';

const comment = (overrides: Partial<IComment> = {}): IComment => ({
	id: '1',
	entity_type: 'tree',
	entity_id: '10',
	added_at: 0,
	added_by: 'user1',
	message: 'Hello',
	...overrides
});

describe('Comment', () => {
	afterEach(cleanup);

	test('renders the system label when the comment has no author', () => {
		const { container } = render(Comment, {
			props: { comment: comment({ added_by: null }) }
		});

		expect(container.textContent).toContain(locale.commentSystem());
	});
});
