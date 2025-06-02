import { apiClient } from '$lib/api';
import { DEFAULT_TREE } from '$lib/constants';

apiClient.getTree = (id: string) => {
	if (id === 'gif2') {
		return new Promise((resolve) => {
			resolve({
				status: 200,
				data: {
					...DEFAULT_TREE,
					species: 'Sequoiadendron giganteum f. pendulum',
					address: 'Barekamutyun, Hrachya Kochar Street, Yerevan',
					users: []
				},
				error: undefined
			});
		});
	}

	return new Promise((resolve) => {
		resolve({
			status: 200,
			data: {
				...DEFAULT_TREE,
				users: []
			},
			error: undefined
		});
	});
};
