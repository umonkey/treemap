import type { Meta, StoryObj } from '@storybook/react';
import { AddTreeDialog } from "./AddTreeDialog";
import { ITreeInfo } from "@/types";

const meta = {
  title: 'Dialogs/AddTreeDialog',
  component: AddTreeDialog,
  parameters: {
    layout: 'padded', // centered, fullscreen
    mockData: [
      {
        url: "http://localhost:8000/v1/trees",
        method: "POST",
        status: 200,
        response: {
          id: 1,
          lat: 40.181389,
          lon: 44.514444,
          name: "Oak",
        } as ITreeInfo,
      },
    ],
  },
  args: { },
} satisfies Meta<typeof AddTreeDialog>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Desktop: Story = {
  args: {
    center: {
      lat: 40.181389,
      lon: 44.514444,
    },
  },
};

export const Phone: Story = {
  args: {
    center: {
      lat: 40.181389,
      lon: 44.514444,
    },
  },
};