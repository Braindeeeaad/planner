export interface Goal {
  id: string;
  nodeId: string;
  title: string;
  targetDate: string;
  status: string;
}

export interface Habit {
  id: string;
  nodeId: string;
  goalId: string | null;
  title: string;
  frequency: string;
  targetTime: number;
  streakCount: number;
}

export interface Task {
  id: string;
  nodeId: string;
  goalId: string | null;
  eventContextId: string | null;
  title: string;
  taskType: string;
  baseDuration: number;
  scheduleStart: string;
  scheduleEnd: string;
  urgencyScore: number;
  importanceScore: number;
  priorityWeight: number;
  status: string;
}

export type NodeType = 'HABIT' | 'GOAL' | 'TASK';

// Simplified Node type for rendering
export interface Node {
  id: string;
  nodeType: NodeType;
  x: number | null;
  y: number | null;
  item: Task | Habit | Goal; // Concrete union instead of generic Action
}


export interface Snapshot {
  nodes: Record<string, Node>;
  successors: Record<string, string[]>;
  predecessors: Record<string, string[]>;
}