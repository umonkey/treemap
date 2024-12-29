import type { Load } from '@sveltejs/kit';
import type { IQuestion } from '$lib/learn/questions';
import { QUESTIONS } from '$lib/learn/questions';
import { shuffle } from '$lib/utils/arrays';

export const load: Load = (): Promise<{
	questions: IQuestion[];
}> => {
	const questions = shuffle(QUESTIONS).map((question: IQuestion) => {
		return {
			options: shuffle(question.options),
			...question
		};
	});

	return {
		questions
	};
};
