// Global imports.
import type { Meta, StoryObj } from '@storybook/react';

// Project imports.
import { WithHeader, WithSidebar, SideBar } from "@/components";
import { ITreeInfo } from "@/types";
import { SAMPLE_TREE } from "@/sample-data";

// Local imports.
import { TreeSidePane } from "./TreeSidePane";

const meta = {
  title: 'Sidebar/TreeSidePane',
  component: TreeSidePane,
  parameters: {
    layout: 'fullscreen',
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
  render: (args) => (
    <WithHeader>
      <WithSidebar>
        <div style={{ width: "100%", padding: "20px" }}>Main contents would be here.</div>
        <SideBar>
          <TreeSidePane {...args} />
        </SideBar>
      </WithSidebar>
    </WithHeader>
  ),
} satisfies Meta<typeof TreeSidePane>;

export default meta;
type Story = StoryObj<typeof meta>;

export const Default: Story = {
  args: {
    tree: SAMPLE_TREE,
  },
};
