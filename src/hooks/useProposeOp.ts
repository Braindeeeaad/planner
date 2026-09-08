import {invoke} from '@tauri-apps/api/core';
import type {Snapshot,Op} from '../types';
import { useCallback } from 'react';

export function useProposeOp() {
  const proposeOp = useCallback(async (op: Op, baseVersion: number) => {
    const result = await invoke<{inverseOp:Array<Op>,version:number,snapshot:Snapshot}>('propose_op', { op, baseVersion });
    return result; // { snapshot, appliedOp, inverseOp }
  }, []);

  return { proposeOp };
}