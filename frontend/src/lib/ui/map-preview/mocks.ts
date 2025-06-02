import { apiClient } from '$lib/api';
import { DEFAULT_TREE } from '$lib/constants';
import { addTrees } from '$lib/stores/treeStore';

apiClient.getTree = (id: string) => {
	if (id === 'gif2') {
		return new Promise((resolve) => {
			const tree = {
				...DEFAULT_TREE,
				id,
				species: 'Sequoiadendron giganteum f. pendulum',
				address: 'Barekamutyun, Hrachya Kochar Street, Yerevan',
				files: [
					{
						id: '286161657200250954',
						small_id: '286161781632667648',
						large_id: '286161786070241280',
						added_at: 1748872283,
						added_by: '284438529478627328'
					},
					{
						id: '286161752813604938',
						small_id: '286161787982843904',
						large_id: '286161790625255424',
						added_at: 1748872306,
						added_by: '284438529478627328'
					},
					{
						id: '286161938889707594',
						small_id: '286162091830808576',
						large_id: '286162095823785984',
						added_at: 1748872350,
						added_by: '284438529478627328'
					},
					{
						id: '286161997568020554',
						small_id: '286162098449420288',
						large_id: '286162102501117952',
						added_at: 1748872364,
						added_by: '284438529478627328'
					},
					{
						id: '286162070829928522',
						small_id: '286162105298718720',
						large_id: '286162108889042944',
						added_at: 1748872382,
						added_by: '284438529478627328'
					}
				],
				users: []
			};

			addTrees([tree]);

			resolve({
				status: 200,
				data: tree,
				error: undefined
			});
		});
	}

	return new Promise((resolve) => {
		const tree = {
			...DEFAULT_TREE,
			id,
			users: []
		};

		addTrees([tree]);

		resolve({
			status: 200,
			data: tree,
			error: undefined
		});
	});
};
