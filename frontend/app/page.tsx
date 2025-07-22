import React from 'react';
import ItemsGrid from '../components/ItemsGrid';
import { useEffect } from 'react';
import useStore from '../store/useStore';
import { io } from 'socket.io-client';

export default function Home() {
  const addItem = useStore((s) => s.addItem);

  useEffect(() => {
    const socket = io(process.env.NEXT_PUBLIC_WS_URL || 'http://localhost:3001');
    socket.on('new-item', (item) => {
      addItem(item);
    });

    return () => {
      socket.disconnect();
    };
  }, [addItem]);

  return (
    <main className="min-h-screen bg-gray-100 p-4">
      <h1 className="text-3xl font-bold mb-4">Brain Rot Roblox Tracker</h1>
      <ItemsGrid />
    </main>
  );
}
