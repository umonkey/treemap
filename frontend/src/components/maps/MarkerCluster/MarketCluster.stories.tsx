import type { Meta, StoryObj } from '@storybook/react';
import { MarkerCluster } from "./MarkerCluster";

import { MapBase, TreeMarker } from "@/components";
import { DEFAULT_MAP_CENTER, DEFAULT_MAP_ZOOM } from "@/utils/config";
import { ITreeInfo } from "@/types";

const WIDTH = 0.05;
const HEIGHT = 0.05;

const getMarkers = () => {
  const markers = [];

  while (markers.length < 1000) {
    const idx = markers.length + 1;
    const lat = DEFAULT_MAP_CENTER.lat + (HEIGHT * Math.random()) - HEIGHT / 2;
    const lon = DEFAULT_MAP_CENTER.lon + (WIDTH * Math.random()) - WIDTH / 2;

    const tree = {
      id: idx.toString(),
      lat,
      lon,
      species: `Tree ${idx}`,
      height: Math.random() * 40,
      circumference: 0.05 + Math.random() * 2,
      diameter: 1.0 + Math.random() * 3,
      state: "healthy",
      updated_at: 1570329700,
      thumbnail_id: null,
    } as ITreeInfo;

    markers.push((
      <TreeMarker tree={tree} />
    ));
  }

  return markers;
};

const meta = {
  title: 'Maps/MarkerCluster',
  component: MarkerCluster,
  parameters: {
    layout: 'fullscreen', // centered
  },
  render: (args) => (
    <MapBase center={DEFAULT_MAP_CENTER} zoom={DEFAULT_MAP_ZOOM}>
      <MarkerCluster {...args}></MarkerCluster>
    </MapBase>
  ),
} satisfies Meta<typeof MarkerCluster>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: {
    children: getMarkers(),
  },
  parameters: {
    viewport: {
      defaultViewport: "responsive",
    },
  },
};

export const Phone: Story = {
  args: {
    children: getMarkers(),
  },
  parameters: {
    viewport: {
      defaultViewport: "mobile1",
    },
  },
};
