import type { Meta, StoryObj } from '@storybook/react';
import { TreeMarkers } from "./TreeMarkers";
import { MapBase } from "@/components";

const CENTER = {
  lat: 40.181389,
  lon: 44.514444,
};

const meta = {
  title: 'Maps/TreeMarkers',
  component: TreeMarkers,
  parameters: {
    layout: 'fullscreen', // centered
  },
  args: { },
  render: () => (
    <MapBase center={CENTER}>
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
