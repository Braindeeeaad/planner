import { useState, useEffect, useCallback } from 'react';
import { fetchSnapshot } from './api/fetchSnapshot';
import type { Snapshot } from './types';
import { ReactFlow, applyNodeChanges, applyEdgeChanges, addEdge } from '@xyflow/react';
import '@xyflow/react/dist/style.css';

const initialNodes = [
  { id: 'n1', position: { x: 0, y: 0 }, data: { label: 'Node 1' } },
  { id: 'n2', position: { x: 0, y: 100 }, data: { label: 'Node 2' } },
];
const initialEdges = [{ id: 'n1-n2', source: 'n1', target: 'n2' }];

// TODO: swap for whatever selects the active goal in your app
const GOAL_ID = 'goal-1';

function snapshotToFlow(snapshot: Snapshot) {
  const nodes = Object.values(snapshot.nodes).map((n) => ({
    id: n.id,
    position: { x: n.x ?? 0, y: n.y ?? 0 },
    data: { label: n.item.title, nodeType: n.nodeType },
  }));

  // successors is an adjacency map: nodeId -> [ids it points to].
  // Flatten it into one edge per (predecessor, successor) pair.
  const edges = Object.entries(snapshot.successors).flatMap(([sourceId, targetIds]) =>
    targetIds.map((targetId) => ({
      id: `${sourceId}-${targetId}`,
      source: sourceId,
      target: targetId,
    }))
  );

  return { nodes, edges };
}

export default function App() {
  const [nodes, setNodes] = useState(initialNodes);
  const [edges, setEdges] = useState(initialEdges);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let cancelled = false;

    async function load() {
      setLoading(true);
      const snapshot = await fetchSnapshot(GOAL_ID);

      if (cancelled) return;

      if (snapshot) {
        const { nodes: flowNodes, edges: flowEdges } = snapshotToFlow(snapshot);
        setNodes(flowNodes);
        setEdges(flowEdges);
      }
      // snapshot === null (fetch failed) -> keep whatever's
      // already in state (placeholder on first load)

      setLoading(false);
    }

    load();
    return () => {
      cancelled = true;
    };
  }, []);

  const onNodesChange = useCallback(
    (changes: any) => setNodes((nodesSnapshot) => applyNodeChanges(changes, nodesSnapshot)),
    []
  );
  const onEdgesChange = useCallback(
    (changes: any) => setEdges((edgesSnapshot) => applyEdgeChanges(changes, edgesSnapshot)),
    []
  );
  const onConnect = useCallback(
    (params: any) => setEdges((edgesSnapshot) => addEdge(params, edgesSnapshot)),
    []
  );

  return (
    <div style={{ width: '100vw', height: '100vh' }}>
      {loading && <div style={{ position: 'absolute', zIndex: 1, padding: 8 }}>Loading…</div>}
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
        fitView
      />
    </div>
  );
}