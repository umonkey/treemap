import type { Load } from '@sveltejs/kit';
import type { IQuestion } from '$lib/learn/questions';
import { QUESTIONS } from '$lib/learn/questions';
import { shuffle } from '$lib/utils/arrays';

export const load: Load = (): Promise<{
	questions: IQuestion[];
}> => {
	const questions = shuffle(QUESTIONS)
		.map((question: IQuestion) => {
			return {
				...question,
				options: shuffle(question.options)
			};
		})
		.slice(0, 10);

	return {
		questions
	};
};
