import type { Meta, StoryObj } from "@storybook/react";

import { AddPage } from "./AddPage";
import { ITreeInfo } from "@/types";

const meta = {
  title: "Pages/AddPage",
  component: AddPage,
  parameters: {
    layout: "padded",
    mockData: [
      {
        url: "/v1/trees",
        method: "POST",
        status: 200,
        response: {
          id: "1",
          lat: 40.181389,
          lon: 44.514444,
          species: "Oak",
        } as ITreeInfo,
      },
    ],
  },
} satisfies Meta<typeof AddPage>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
};
