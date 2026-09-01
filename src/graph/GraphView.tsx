import React, { useCallback, useState } from "react";
import {
  ReactFlow,
  ReactFlowProvider,
  applyNodeChanges,
  applyEdgeChanges,
  addEdge,
} from "@xyflow/react";
import type {
  Node,
  Edge,
  NodeChange,
  EdgeChange,
  Connection,
} from "@xyflow/react";
import DagreNodePositioning, {
  type DagreGraphOptions,
} from "../layout/DagreNodePositioning";
import "@xyflow/react/dist/style.css";

interface GraphViewProps {
  nodes: Node[];
  edges: Edge[];
  setNodes: React.Dispatch<React.SetStateAction<Node[]>>;
  setEdges: React.Dispatch<React.SetStateAction<Edge[]>>;
  loading?: boolean;
  layoutOptions?: DagreGraphOptions;
}

const defaultLayoutOptions: DagreGraphOptions = {
  rankdir: "TB",
  nodesep: 50,
  ranksep: 50,
};

export const GraphView: React.FC<GraphViewProps> = ({
  nodes,
  edges,
  setNodes,
  setEdges,
  loading = false,
  layoutOptions = defaultLayoutOptions,
}) => {
  const [, setViewIsFit] = useState<boolean>(false);

  const onNodesChange = useCallback(
    (changes: NodeChange[]) =>{
      setNodes((nodesSnapshot) => applyNodeChanges(changes, nodesSnapshot));
      
    },  
    [setNodes]
  );

  const onEdgesChange = useCallback(
    (changes: EdgeChange[]) =>
      setEdges((edgesSnapshot) => applyEdgeChanges(changes, edgesSnapshot)),
    [setEdges]
  );

  const onConnect = useCallback(
    (params: Connection) =>
      setEdges((edgesSnapshot) => addEdge(params, edgesSnapshot)),
    [setEdges]
  );

  return (
    <div style={{ width: "100vw", height: "100vh", position: "relative" }}>
      {loading && (
        <div style={{ position: "absolute", zIndex: 1, padding: 8 }}>
          Loading…
        </div>
      )}
      <ReactFlowProvider>
        <DagreNodePositioning
          Options={layoutOptions}
          SetNodes={setNodes}
          SetEdges={setEdges}
          Edges={edges}
          SetViewIsFit={setViewIsFit}
        />
        <ReactFlow
          nodes={nodes}
          edges={edges}
          onNodesChange={onNodesChange}
          onEdgesChange={onEdgesChange}
          onConnect={onConnect}
          fitView
        />
      </ReactFlowProvider>
    </div>
  );
};