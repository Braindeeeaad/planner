import {invoke} from '@tauri-apps/api/core';
import type {Snapshot,Op} from '../types';
import { useCallback } from 'react';

export function useProposeOp() {
  const proposeOp = useCallback(async (op: Op, baseVersion: number) => {
    const result = await invoke<Array<Op>>('propose_op', { op, baseVersion });
    return result; // { snapshot, appliedOp, inverseOp }
  }, []);

  return { proposeOp };
}