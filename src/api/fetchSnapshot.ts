import {invoke} from '@tauri-apps/api/core';
import type {Snapshot} from '../types';

export async function fetchSnapshot(goalId: string): Promise<Snapshot | null> {
  try {
    const snapshot = await invoke<Snapshot>('get_snapshot', {
      goalId: goalId // Rust receives this as `goal_id`
    });
    console.log('Fetched snapshot:', snapshot);
    return snapshot;
  } catch (error) {
    console.error('Failed to fetch snapshot:', error);
    return null;
  }
}