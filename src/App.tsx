import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import Canvas from './components/Canvas';
import { Body } from './types';
import './App.css';

function App() {
    const [bodies, setBodies] = useState<Body[]>([]);

    useEffect(() => {
        async function fetchBodies() {
            try {
                const result = await invoke<Body[]>('get_bodies');
                setBodies(result);
            } catch (error) {
                console.error('Failed to fetch bodies:', error);
            }
        }

        async function update() {
            await fetchBodies();
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
        <div className="row">
            <Canvas bodies={bodies} />
        </div>
        </>
    );
}

export default App;

