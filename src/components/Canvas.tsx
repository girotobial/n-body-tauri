import React, { useEffect, useRef } from 'react';
import { Body } from '../types';

type CanvasProps = {
    bodies: Body[]
}

const Canvas: React.FC<CanvasProps> = ({ bodies }) => {
    const canvasRef = useRef<HTMLCanvasElement | null>(null);

    useEffect(() => {
        const canvas = canvasRef.current;
        if (!canvas) return;

        const context = canvas.getContext('2d');
        if (!context) return;

        // Set canvas dimensions
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;

        // Clear the canvas

        const draw = (body: Body): void => {
            context.beginPath();
            context.arc(body.position.x, body.position.y, 1, 0, 2 * Math.PI);
            context.fillStyle = 'red';
            context.fill();
            context.stroke();
        }

        const update = () => {
            context.clearRect(0, 0, canvas.width, canvas.height);

            bodies.forEach(body => {
                draw(body);
            });
        }

        update()

        // Draw bodies
    }, [bodies]);


    return <canvas ref={canvasRef} />;
};

export default Canvas;
