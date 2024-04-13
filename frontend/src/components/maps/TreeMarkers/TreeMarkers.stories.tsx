import type { Meta, StoryObj } from '@storybook/react';
import { TreeMarkers } from "./TreeMarkers";
import { MapBase } from "@/components";
import { DEFAULT_MAP_CENTER, DEFAULT_MAP_ZOOM } from "@/utils/config";

const meta = {
  title: 'Maps/TreeMarkers',
  component: TreeMarkers,
  parameters: {
    layout: 'fullscreen', // centered
  },
  args: { },
  render: () => (
    <MapBase center={DEFAULT_MAP_CENTER} zoom={DEFAULT_MAP_ZOOM}>
      <TreeMarkers />
    </MapBase>
  ),
} satisfies Meta<typeof TreeMarkers>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: { },
  parameters: {
    viewport: {
      defaultViewport: "responsive",
    },
  },
};

export const Phone: Story = {
  args: { },
  parameters: {
    viewport: {
      defaultViewport: "mobile1",
    },
  },
};
