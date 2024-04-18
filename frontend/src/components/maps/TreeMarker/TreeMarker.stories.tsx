import type { Meta, StoryObj } from '@storybook/react';
import { TreeMarker } from "./TreeMarker";
import { MapBase } from "@/components";
import { DEFAULT_MAP_CENTER, DEFAULT_MAP_ZOOM } from "@/utils/config";
import { ITreeInfo } from "@/types";

const TREE = {
  id: "12345",
  lat: 40.181389,
  lon: 44.514444,
  name: "Big Oak",
  height: 15.0,
  circumference: 1.6,
  diameter: 4.5,
  state: "healthy",
  updated_at: 1713446115,
  thumbnail_id: null,
} as ITreeInfo;

const meta = {
  title: 'Maps/TreeMarker',
  component: TreeMarker,
  parameters: {
    layout: 'fullscreen', // centered
  },
  args: { },
  render: (args) => (
    <MapBase center={DEFAULT_MAP_CENTER} zoom={DEFAULT_MAP_ZOOM}>
      <TreeMarker {...args} />
    </MapBase>
  ),
} satisfies Meta<typeof TreeMarker>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: {
    tree: TREE,
  },
  parameters: {
    viewport: {
      defaultViewport: "responsive",
    },
  },
};

export const Phone: Story = {
  args: {
    tree: TREE,
  },
  parameters: {
    viewport: {
      defaultViewport: "mobile1",
    },
  },
};
