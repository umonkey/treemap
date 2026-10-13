export interface ISpeciesPhotoSet {
	species: string[];
	photos: string[];
}

const SPECIES_PHOTOS: ISpeciesPhotoSet[] = [
	{
		species: ['ulmus glabra'],
		photos: [
			'https://www.vdberk.co.uk/media/88960/ulmus-glabra.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/7/76/Ulmus_glabra.jpg'
		]
	},
	{
		species: ['ulmus'],
		photos: [
			'https://thumb.wikimedia.org/wikipedia/commons/thumb/4/43/Blismes_elm_2007.jpg/960px-Blismes_elm_2007.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/5/5c/Ulmus_minor._Negrillu.jpg'
		]
	},
	{
		species: ['catalpa'],
		photos: [
			'https://upload.wikimedia.org/wikipedia/commons/7/71/Catalpa_bignonioides_Walt_Stockholm_Karlaplan_2006-07-19.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/e/e2/Catalpa_bignonioides_JPG5Aa.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/c/c6/Catalpa_bignonioides_inflorescence.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/9/9a/Catalpa_bignoioides_004.JPG'
		]
	},
	{
		species: ['catalpa bignonioides nana'],
		photos: ['https://pepinierestaubin.ca/wp-content/uploads/2026/03/Catalpa-Bignonioides-Nana.jpg']
	},
	{
		species: ['fraxinus'],
		photos: [
			'https://www.treesandshrubsonline.org/site/assets/files/9846/fraxinus-angustifolia-f-oxycarpa.jpg',
			'https://www.treesandshrubsonline.org/site/assets/files/9845/fraxinus-angustifolia-var-lentiscifolia-1.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/b/b8/Fraxinus_angustifolia_foliage.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/4/4f/Fraxinus_excelsior.jpg'
		]
	},
	{
		species: ['platanus', 'platanus orientalis'],
		photos: [
			'https://upload.wikimedia.org/wikipedia/commons/e/ea/Star%C3%BD_platan%2C_N%C3%A1horn%C3%AD_Karabach.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/3/37/Platanus_orientalis_infructescence_UC_Davis_3.jpg'
		]
	},
	{
		species: ['robinia', 'robinia pseudoacacia'],
		photos: [
			'https://upload.wikimedia.org/wikipedia/commons/f/ff/Rajhrad_-_Benediktinsk%C3%BD_kl%C3%A1%C5%A1ter_-_ak%C3%A1t.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/2/2d/Robinia_(Robinia_pseudoacacia).jpg',
			'https://upload.wikimedia.org/wikipedia/commons/e/e3/Robinia_pseudacacia_seeds.jpg'
		]
	},
	{
		species: ['paulownia'],
		photos: [
			'https://upload.wikimedia.org/wikipedia/commons/7/74/Paulownia_tomentosa_JPG2a.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/3/35/Baum280906.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/d/d1/Paulownia_tomentosa1.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/b/b8/Paulownia-fruit1.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/e/e1/Paulownia_tomentosa4.jpg'
		]
	},
	{
		species: ['populus alba'],
		photos: [
			'https://upload.wikimedia.org/wikipedia/commons/b/b3/20110714Populus_alba1.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/d/df/Naturdenkmal_782_sl1.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/9/9c/Populus_alba_leaf.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/0/05/Populus_alba_002.JPG'
		]
	},
	{
		species: ['populus nigra'],
		photos: [
			'https://upload.wikimedia.org/wikipedia/commons/4/40/PopulusNigra5.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/8/89/Potsdam-Freundschaftsinsel-Populus-nigra-Italica-08-VII-2007-096.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/2/2e/PopulusNigra4c.jpg'
		]
	},
	{
		species: ['quercus', 'quercus robur'],
		photos: [
			'https://upload.wikimedia.org/wikipedia/commons/3/3b/Eiche_am_Ortseingang_zu_Martinfeld_im_Eichsfeld.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/3/35/Brockwell_Oak_(9).jpg',
			'https://upload.wikimedia.org/wikipedia/commons/a/af/Quercus_robur.jpg'
		]
	},
	{
		species: ['quercus macranthera'],
		photos: [
			'https://www.biolib.cz/IMG/GAL/BIG/379977.jpg',
			'https://www.vdberk.co.uk/media/76430/quercus-macranthera.jpg',
			'https://www.treesandshrubsonline.org/site/assets/files/7088/quercus-macranthera-1.jpg',
			'https://www.plantarium.ru/dat/plants/9/984/160984_3ea5aa83.jpg'
		]
	},
	{
		species: ['quercus rubra'],
		photos: [
			'https://upload.wikimedia.org/wikipedia/commons/0/05/Quercus_rubra_(northern_red_oak)_1.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/9/98/A_Red_oak_(Quercus_rubra)_-_young_leaves_-_geograph.org.uk_-_5762146.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/8/85/N_Red_Oak_9386.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/c/c2/Quercus_rubra%2C_Blatt%2C_-freigestellt-_remixed.png'
		]
	},
	{
		species: ['quercus castaneifolia'],
		photos: [
			'https://upload.wikimedia.org/wikipedia/commons/4/4e/Ham_Common%2C_view_of_the_pond_with_the_chestnut-leaved_oak.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/7/76/Quercus_castaneifolia2_kew.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/3/3e/Chestnut_tree_in_Ilam_province.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/2/26/Quercus_castaneifolia_kz03.jpg'
		]
	},
	{
		species: ['cercis', 'cercis canadensis'],
		photos: [
			'https://upload.wikimedia.org/wikipedia/commons/9/98/A18-1-Cercis_Canadensis_(Eastern_Redbud).JPG',
			'https://upload.wikimedia.org/wikipedia/commons/b/b5/Cercis_canadensis_in_botanical_garden_of_UKW_Bydgoszcz_(5)_V.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/b/b9/Cercis_canadensis_Judaszowiec_kanadyjski_2020-08-05_01.jpg'
		]
	},
	{
		species: ['cercis siliquastrum'],
		photos: [
			'https://upload.wikimedia.org/wikipedia/commons/1/1c/Cercis_siliquastrum_in_Golden_Valley_Tree_Park%2C_May_2022_01.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/c/c0/Arbre_de_Jud%C3%A9e_FR_2012.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/b/b9/Cercis_siliquastrum_2_RF.jpg'
		]
	},
	{
		species: ['acer negundo'],
		photos: [
			'https://upload.wikimedia.org/wikipedia/commons/e/e3/Acer_negundo_tree.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/9/9f/2020_year._Herbarium._Acer_negundo._img-010.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/a/af/2020_year._Herbarium._Acer_negundo._img-031.jpg'
		]
	},
	{
		species: ['acer campestre'],
		photos: [
			'https://upload.wikimedia.org/wikipedia/commons/d/df/Acer_campestre_006.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/2/27/Acer_campestre_02_by-dpc.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/d/db/Acer_campestre_L95.jpg'
		]
	},
	{
		species: ['acer palmatum'],
		photos: [
			'https://upload.wikimedia.org/wikipedia/commons/4/40/Acer_palmatum_subsp._amoenum_in_Hackfalls_Arboretum_(6).jpg',
			'https://upload.wikimedia.org/wikipedia/commons/d/d0/Acer_palmatum_(Japanese_Maple)%2C_Toccoa%2C_Georgia.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/c/cd/Hortus_Haren_(Groningen)_20-04-2025._(actm.)_13.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/2/25/Acer_palmatum_plant.jpg'
		]
	},
	{
		species: ['acer pseudoplatanus'],
		photos: [
			'https://upload.wikimedia.org/wikipedia/commons/0/0a/Lajen_Ahorn_und_Ansitz_Fonteklaus_Freins.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/5/5c/2018-06-07_Sycamore_Gap_Tree_(Acer_pseudoplatanus)%2C_next_to_Hadrian%E2%80%99s_Wall_UK.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/2/29/Acer_pseudoplatanus_122067251.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/5/5a/Acer_pseudoplatanus_3_RF.jpg'
		]
	},
	{
		species: ['thuja', 'thuja occidentalis'],
		photos: [
			'https://upload.wikimedia.org/wikipedia/commons/7/74/Thuja_occidentalis_at_a_memorial_in_M%C3%A1laga.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/c/ca/Thuja_occidentalis_in_Komsomolsky_Square_-_Efremov%2C_Russia2.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/1/1e/Thuja_occidentalis_qtl1.jpg'
		]
	},
	{
		species: ['ailanthus', 'ailanthus altissima'],
		photos: [
			'https://upload.wikimedia.org/wikipedia/commons/e/eb/G%C3%B6tterbaum_(Ailanthus_altissima).jpg',
			'https://upload.wikimedia.org/wikipedia/commons/8/81/Ailanthus_altissima_20200525a.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/2/2e/Ailanthus_altissima_20200721a.jpg'
		]
	},
	{
		species: ['morus', 'morus alba'],
		photos: [
			'https://upload.wikimedia.org/wikipedia/commons/8/8a/FruitlessMulberry-3965.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/b/b5/Morus_alba-leaves.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/0/0a/Morus-alba.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/1/19/Morus_alba_(White_mulberry).jpg'
		]
	},
	{
		species: ['albizia', 'albizia julibrissin'],
		photos: [
			'https://thumb.wikimedia.org/wikipedia/commons/thumb/6/66/2026_Albizia_julibrissin_no_Campus_Sur_da_USC._Santiago_de_Compostela._Galiza.jpg/1280px-2026_Albizia_julibrissin_no_Campus_Sur_da_USC._Santiago_de_Compostela._Galiza.jpg',
			"upload.wikimedia.org/wikipedia/commons/7/72/Albizia_julibrissin_'Boubri'_2020-07-18_02.jpg",
			'https://upload.wikimedia.org/wikipedia/commons/c/c9/Albizia_julibrissin_flower.jpg'
		]
	},
	{
		species: ['aesculus', 'aesculus hippocastanum'],
		photos: [
			'https://upload.wikimedia.org/wikipedia/commons/7/7b/Casta%C3%B1o_de_Indias_(Aesculus_hippocastanum)%2C_Sopot%2C_Polonia%2C_2013-05-22%2C_DD_01.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/4/45/Aesculus_hippocastanum_2018-04-27_9488.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/1/14/Aesculus_hippocastanum_fruit.jpg'
		]
	},
	{
		species: ['koelreuteria', 'koelreuteria paniculata'],
		photos: [
			'https://upload.wikimedia.org/wikipedia/commons/d/db/Koelreuteria_paniculata_in_Jingshan_Park.jpg',
			'https://thumb.wikimedia.org/wikipedia/commons/thumb/b/b1/Koelreuteria_paniculata_Roztrzeplin_wiechowaty_2024-05-26_03.jpg/1280px-Koelreuteria_paniculata_Roztrzeplin_wiechowaty_2024-05-26_03.jpg',
			'https://thumb.wikimedia.org/wikipedia/commons/thumb/0/09/(MHNT)_Koelreuteria_paniculata_-Leaves_and_inflorescences_-_Palais_Niel.jpg/1280px-(MHNT)_Koelreuteria_paniculata_-Leaves_and_inflorescences_-_Palais_Niel.jpg',
			'https://thumb.wikimedia.org/wikipedia/commons/thumb/2/23/Koelreuteria_paniculata%2C_Santa_Coloma_de_Farners_02.jpg/1280px-Koelreuteria_paniculata%2C_Santa_Coloma_de_Farners_02.jpg'
		]
	},
	{
		species: ['juniperus', 'juniperus virginiana'],
		photos: [
			'https://upload.wikimedia.org/wikipedia/commons/2/20/Juniperus_virginiana_near_Oxford%2C_Ohio.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/7/72/Juniperus_virginiana%2C_Yerevan_Botanical_Garden.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/c/cd/Juniperus_virginiana%2C_Belgrade.jpg'
		]
	},
	{
		species: ['carpinus'],
		photos: [
			'https://upload.wikimedia.org/wikipedia/commons/d/d7/GOC_Bengeo_to_Woodhall_Park_125_Hornbeam_(Carpinus_betulus)_(8106651650).jpg',
			"https://upload.wikimedia.org/wikipedia/commons/7/7a/Carpinus_betulus_'Fastigiata'_Dublin_P1310089.jpg",
			'https://upload.wikimedia.org/wikipedia/commons/b/b3/Carpinus_orientalis_Oriental_Hornbeam_%E1%83%AF%E1%83%90%E1%83%92%E1%83%A0%E1%83%AA%E1%83%AE%E1%83%98%E1%83%9A%E1%83%90.JPG',
			'https://upload.wikimedia.org/wikipedia/commons/1/1d/Carpinus_betulus_all.jpg'
		]
	},
	{
		species: ['juglans', 'juglans regia'],
		photos: [
			'https://upload.wikimedia.org/wikipedia/commons/3/35/Juglans-regia-total.JPG',
			'https://thumb.wikimedia.org/wikipedia/commons/thumb/9/94/Blooming_walnut_(Juglans_regia).jpg/1280px-Blooming_walnut_(Juglans_regia).jpg',
			'https://thumb.wikimedia.org/wikipedia/commons/thumb/1/1b/Juglans_regia_2009_G2.jpg/1280px-Juglans_regia_2009_G2.jpg'
		]
	},
	{
		species: ['pinus pinea'],
		photos: [
			"https://thumb.wikimedia.org/wikipedia/commons/thumb/5/50/R'as_Al-matn_pine_forest-Lebanon.jpg/1280px-R'as_Al-matn_pine_forest-Lebanon.jpg",
			'https://thumb.wikimedia.org/wikipedia/commons/thumb/d/d8/Pinus_pinea_Borghese.jpg/1280px-Pinus_pinea_Borghese.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/0/01/Pinus_pinea_foliage.jpg'
		]
	},
	{
		species: ['pinus'],
		photos: [
			'https://upload.wikimedia.org/wikipedia/commons/5/59/%C5%9Awierklaniec_pinus_nigra_12.06.2010_p.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/4/41/Luxembourg_Berschbach_Pinus_nigra_2008.jpg',
			'https://thumb.wikimedia.org/wikipedia/commons/thumb/6/61/Jard%C3%ADn_Bot%C3%A1nico_Olarizu_-_Pinus_nigra_-_Hojas_y_pi%C3%B1a_01.jpg/1280px-Jard%C3%ADn_Bot%C3%A1nico_Olarizu_-_Pinus_nigra_-_Hojas_y_pi%C3%B1a_01.jpg'
		]
	},
	{
		species: ['styphnolobium', 'styphnolobium japonicum'],
		photos: [
			'https://upload.wikimedia.org/wikipedia/commons/e/e4/Styphnolobium_japonicum_1347.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/c/c0/Styphnolobium_japonicum_1348.jpg',
			'https://thumb.wikimedia.org/wikipedia/commons/thumb/8/8f/Styphnolobium_japonicum_1DS-II_4-4333.jpg/1280px-Styphnolobium_japonicum_1DS-II_4-4333.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/9/9b/Styphnolobium_japonicum_Pere%C5%82kowiec_japo%C5%84ski_2018-09-02_01.jpg'
		]
	},
	{
		species: ['malus', 'malus domestica'],
		photos: [
			'https://thumb.wikimedia.org/wikipedia/commons/thumb/e/ee/Malus_domestica_(Apple)_flowers_2.jpg/1280px-Malus_domestica_(Apple)_flowers_2.jpg'
		]
	},
	{
		species: ['liriodendron', 'liriodendron tulipifera'],
		photos: [
			'https://upload.wikimedia.org/wikipedia/commons/1/17/Liriodendron_tulipifera_Tulipanowiec_ameryka%C5%84ski_Grenoble_2023-03-01_01.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/a/af/Liriodendron_tulipifera_JPG5a.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/0/0e/Liriodendron_tulipifera_-_Tulpenbaum_-_Rheinpark_Koeln.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/f/f0/Tulip_Tree_Liriodendron_tulipifera_Leaf_Topside_1890px.jpg'
		]
	},
	{
		species: ['magnolia'],
		photos: [
			"https://thumb.wikimedia.org/wikipedia/commons/thumb/f/f7/Magnolie_'Donna'_im_botanischen_Garten_M%C3%BCnchen-Nymphenburg_01.jpg/1280px-Magnolie_'Donna'_im_botanischen_Garten_M%C3%BCnchen-Nymphenburg_01.jpg",
			'https://upload.wikimedia.org/wikipedia/commons/4/48/Magnolia_%C3%97_soulangeana_blossom.jpg'
		]
	},
	{
		species: ['cupressus cashmeriana'],
		photos: [
			'https://thumb.wikimedia.org/wikipedia/commons/thumb/b/b5/Cupressus_cashmeriana%2C_Wangdue_Phodrang%2C_Bhutan.jpg/1920px-Cupressus_cashmeriana%2C_Wangdue_Phodrang%2C_Bhutan.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/2/2a/%E5%96%80%E4%BB%80%E7%B1%B3%E7%88%BE%E6%9F%8F_Cupressus_cashmeriana_20220126102449_05.jpg',
			'https://thumb.wikimedia.org/wikipedia/commons/thumb/1/16/Cupressus_cashmeriana_CBN_Brest_05.jpg/1280px-Cupressus_cashmeriana_CBN_Brest_05.jpg'
		]
	},
	{
		species: ['lagerstroemia', 'lagerstroemia indica'],
		photos: [
			'https://thumb.wikimedia.org/wikipedia/commons/thumb/1/17/Lagerstroemia_indica_MHNT_Jardin_des_Plantes_de_Toulouse.jpg/1920px-Lagerstroemia_indica_MHNT_Jardin_des_Plantes_de_Toulouse.jpg',
			'https://thumb.wikimedia.org/wikipedia/commons/thumb/d/d4/Lagerstroemia_indica_5.jpg/1920px-Lagerstroemia_indica_5.jpg',
			'https://upload.wikimedia.org/wikipedia/commons/5/53/Lagerstroemia_indica_Blanco1.207.png'
		]
	},
	{
		species: ['maclur', 'maclura', 'maclur pomofera', 'maclura pomifera'],
		photos: [
			"https://thumb.wikimedia.org/wikipedia/commons/thumb/3/37/Maclura_pomifera_arbre_d'inter%C3%A8s_local_Parc_de_la_Ciutadella_P2080013_-_Flickr_-_tgrauros.jpg/1920px-Maclura_pomifera_arbre_d'inter%C3%A8s_local_Parc_de_la_Ciutadella_P2080013_-_Flickr_-_tgrauros.jpg",
			'https://thumb.wikimedia.org/wikipedia/commons/thumb/e/e8/Maclura_pomifera_%C5%BB%C3%B3%C5%82tnica_pomara%C5%84czowa_2021-10-02_03.jpg/1280px-Maclura_pomifera_%C5%BB%C3%B3%C5%82tnica_pomara%C5%84czowa_2021-10-02_03.jpg',
			'https://thumb.wikimedia.org/wikipedia/commons/thumb/8/8e/Maclura_pomifera_Rovato.jpg/1280px-Maclura_pomifera_Rovato.jpg'
		]
	},
	{
		species: ['metasequoia', 'metasequoia glyptostroboides', 'metasequoia glyphtostroboides'],
		photos: [
			'https://upload.wikimedia.org/wikipedia/commons/4/47/Metasequoia_glyptostroboides_in_Forest_park_of_Aichi_Prefecture.jpg',
			'https://thumb.wikimedia.org/wikipedia/commons/thumb/b/b4/Metasequoia_glyptostroboides_(1)_Meise_Nat._Plantentuin.jpg/1280px-Metasequoia_glyptostroboides_(1)_Meise_Nat._Plantentuin.jpg'
		]
	}
];

const normalize = (value: string): string => value.trim().toLowerCase().replace(/\s+/g, ' ');

export const findSpeciesPhotos = (species: string | null | undefined): string[] => {
	const key = normalize(species ?? '');

	if (!key || key === 'unknown') {
		return [];
	}

	return [
		...new Set(
			SPECIES_PHOTOS.filter((set) => set.species.some((s) => normalize(s) === key)).flatMap(
				(set) => set.photos
			)
		)
	];
};
