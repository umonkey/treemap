export const lang = {
	title: 'Laser Range Triangulation - Enter Distances',
	intro:
		'Enter the measured laser distance (in meters) from each Ground Control Point to the target tree.',
	gcpRadiusLabel: (index: number, lat: number, lng: number) =>
		`GCP ${index} (${lat.toFixed(5)}, ${lng.toFixed(5)}) distance (m)`,
	suggestedLocation: 'Suggested Tree Location:',
	back: 'Back to Setup',
	saveTree: 'Add Tree at Triangulated Location'
};
