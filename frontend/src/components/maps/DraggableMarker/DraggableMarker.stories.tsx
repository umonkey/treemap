// Global imports.
import type { Meta, StoryObj } from '@storybook/react';
import { fn } from '@storybook/test';

// Project imports.
import { DEFAULT_MAP_CENTER, DEFAULT_MAP_ZOOM } from "@/utils/config";
import { MapBase } from "@/components";

// Local imports.
import { DraggableMarker } from "./DraggableMarker";

const meta = {
  title: 'Maps/DraggableMarker',
  component: DraggableMarker,
  parameters: {
    layout: 'fullscreen', // centered
  },
  args: {
    onChange: fn(),
  },
  render: (args) => (
    <MapBase center={DEFAULT_MAP_CENTER} zoom={DEFAULT_MAP_ZOOM}>
      <DraggableMarker {...args} />
    </MapBase>
  ),
} satisfies Meta<typeof DraggableMarker>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    center: DEFAULT_MAP_CENTER,
  },
};
