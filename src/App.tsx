import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import Canvas from './components/Canvas';
import { Body, Tree } from './types';
import './App.css';

function App() {
    const [bodies, setBodies] = useState<Body[]>([]);
    const [tree, setTree] = useState<Tree>({boundaries: [], center_of_mass: {x: 0, y: 0}});

    useEffect(() => {
        async function fetchBodies() {
            try {
                const result = await invoke<Body[]>('get_bodies');
                setBodies(result);
            } catch (error) {
                console.error('Failed to fetch bodies:', error);
            }
        }
        async function fetchTree() {
            try {
                const result = await invoke<Tree>('get_tree');
                setTree(result);
            } catch (error) {
                console.error('Failed to fetch tree:', error);
            }
        }

        async function update() {
            await fetchBodies();
            await fetchTree();
        }

        const animate = () => {
            update();
            requestAnimationFrame(animate);
        }

        requestAnimationFrame(animate);
        return () => {
            // @ts-ignore
            cancelAnimationFrame(animate);
        };

    }, []);

    return (
        <>
        <h1>N-Body Problem</h1>
            <Canvas bodies={bodies} tree={tree} />
        </>
    );
}

export default App;

