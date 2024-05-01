// Global imports.
import type { Meta, StoryObj } from '@storybook/react';

// Project imports.
import { DEFAULT_MAP_CENTER, DEFAULT_MAP_ZOOM } from "@/utils/config";
import { MapBase } from "@/components";

// Local imports.
import { AddTreeControl } from "./AddTreeControl";

const meta = {
  title: 'Maps/AddTreeControl',
  component: AddTreeControl,
  parameters: {
    layout: 'fullscreen', // centered
  },
  args: {
    position: "bottomright",
  },
  render: (args) => (
    <MapBase center={DEFAULT_MAP_CENTER} zoom={DEFAULT_MAP_ZOOM}>
      <AddTreeControl {...args} />
    </MapBase>
  ),
} satisfies Meta<typeof AddTreeControl>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
};
