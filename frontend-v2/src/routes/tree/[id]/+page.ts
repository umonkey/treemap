import type { Load } from '@sveltejs/kit';

export const load: Load = async ({ params }) => {
	return {
		id: params.id,

		tree: {
			id: params.id,
			lat: 40.17992075412105,
			lon: 44.5093059539795,
			osm_id: null,
			species: 'Fraxinus',
			notes:
				'The General Sherman tree was named after the American Civil War general William Tecumseh Sherman. The official story, which may be apocryphal, claims the tree was named in 1879 by naturalist James Wolverton, who had served as a lieutenant in the 9th Indiana Cavalry under Sherman.\n\nГенерал Шерман (англ. General Sherman) — экземпляр секвойядендрона гигантского (Sequoiadendron giganteum), растущий в «Гигантском лесу» национального парка «Секвойя» в Калифорнии, США. Первое по объёму и массе дерево на Земле. Высота «Генерала Шермана» — 85 м (по состоянию на 2018 год), масса оценивается в 1910 т (оценка 1938 года), объём ствола — в 1487 м³, возраст — в 2300—2700 лет.',
			height: 0.0,
			circumference: 0.68,
			diameter: 5.0,
			state: 'healthy',
			added_at: 1718132083,
			updated_at: 1718132342,
			added_by: '134022734140280832',
			thumbnail_id: '157235717817372672',
			files: [
				{
					id: '157235708124336128',
					small_id: '157235717817372672',
					large_id: '157235721479000064'
				},
				{
					id: '157235704613703680',
					small_id: '157235711735631872',
					large_id: '157235715229487104'
				},
				{
					id: '157235700834635776',
					small_id: '157235705318346752',
					large_id: '157235709017722880'
				}
			]
		}
	};
};
