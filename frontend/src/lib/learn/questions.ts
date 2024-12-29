export interface IQuestion {
	question: string;
	image: string;
	options: string[];
	correct: string[];
}

export const QUESTIONS: IQuestion[] = [
	{
		question: 'Какое дерево на картинке?',
		image: '/learn/maple-leaf.jpg',
		options: ['клён', 'каштан', 'платан', 'берёза', 'дуб'],
		correct: ['клён']
	},
	{
		question: 'Какое дерево на картинке?',
		image: '/learn/ash-leaf.jpg',
		options: ['ясень', 'рябина', 'орех', 'айлант', 'акация'],
		correct: ['ясень']
	}
];
