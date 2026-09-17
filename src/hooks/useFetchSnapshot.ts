import {invoke} from '@tauri-apps/api/core';
import type {SnapshotVersion} from '../types';
import { useCallback } from 'react';

export function useFetchSnapshot(){
    const graphSnapshot = useCallback(async(goalId:string)=>{
        const snapshot = await invoke<SnapshotVersion>('get_snapshot', {
            goalId: goalId // Rust receives this as `goal_id`
        });
        return snapshot;
    }, []); 
    return {graphSnapshot};
}
