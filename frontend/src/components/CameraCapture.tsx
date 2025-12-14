'use client';

import { useRef, useState, useCallback, useEffect } from 'react';
import { CaptureMetadata } from '@/types/capture';

interface CameraCaptureProps {
  onCapture: (imageData: string, metadata: CaptureMetadata) => void;
  onError?: (error: string) => void;
}

export function CameraCapture({ onCapture, onError }: CameraCaptureProps) {
  const videoRef = useRef<HTMLVideoElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const streamRef = useRef<MediaStream | null>(null);
  
  const [isStreaming, setIsStreaming] = useState(false);
  const [facingMode, setFacingMode] = useState<'user' | 'environment'>('environment');
  const [error, setError] = useState<string | null>(null);
  const [capturedImage, setCapturedImage] = useState<string | null>(null);

  const startCamera = useCallback(async () => {
    try {
      setError(null);
      
      if (streamRef.current) {
        streamRef.current.getTracks().forEach(track => track.stop());
      }

      const constraints: MediaStreamConstraints = {
        video: {
          facingMode: facingMode,
          width: { ideal: 1920 },
          height: { ideal: 1080 },
        },
        audio: false,
      };

      const stream = await navigator.mediaDevices.getUserMedia(constraints);
      streamRef.current = stream;

      if (videoRef.current) {
        videoRef.current.srcObject = stream;
        await videoRef.current.play();
        setIsStreaming(true);
      }
    } catch (err) {
      const message = err instanceof Error ? err.message : 'Failed to access camera';
      setError(message);
      onError?.(message);
    }
  }, [facingMode, onError]);

  const stopCamera = useCallback(() => {
    if (streamRef.current) {
      streamRef.current.getTracks().forEach(track => track.stop());
      streamRef.current = null;
    }
    if (videoRef.current) {
      videoRef.current.srcObject = null;
    }
    setIsStreaming(false);
  }, []);

  const toggleCamera = useCallback(() => {
    setFacingMode(prev => prev === 'user' ? 'environment' : 'user');
  }, []);

  useEffect(() => {
    if (isStreaming) {
      startCamera();
    }
  }, [facingMode]);

  useEffect(() => {
    return () => {
      stopCamera();
    };
  }, [stopCamera]);

  const captureImage = useCallback(() => {
    if (!videoRef.current || !canvasRef.current) return;

    const video = videoRef.current;
    const canvas = canvasRef.current;
    
    canvas.width = video.videoWidth;
    canvas.height = video.videoHeight;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    ctx.drawImage(video, 0, 0);
    
    const imageData = canvas.toDataURL('image/jpeg', 0.9);
    setCapturedImage(imageData);

    const metadata: CaptureMetadata = {
      capturedAt: new Date().toISOString(),
      facingMode: facingMode,
      resolution: {
        width: video.videoWidth,
        height: video.videoHeight,
      },
    };

    onCapture(imageData, metadata);
  }, [facingMode, onCapture]);

  const retake = useCallback(() => {
    setCapturedImage(null);
  }, []);

  return (
    <div className="flex flex-col items-center gap-4" data-testid="camera-capture">
      <div className="relative w-full max-w-xl aspect-[4/3] bg-gray-900 rounded-lg overflow-hidden">
        {!isStreaming && !capturedImage && (
          <div className="absolute inset-0 flex flex-col items-center justify-center text-gray-400">
            <svg className="w-16 h-16 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 9a2 2 0 012-2h.93a2 2 0 001.664-.89l.812-1.22A2 2 0 0110.07 4h3.86a2 2 0 011.664.89l.812 1.22A2 2 0 0018.07 7H19a2 2 0 012 2v9a2 2 0 01-2 2H5a2 2 0 01-2-2V9z" />
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 13a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
            <p>Camera not started</p>
          </div>
        )}

        {capturedImage ? (
          <img 
            src={capturedImage} 
            alt="Captured card" 
            className="w-full h-full object-contain"
            data-testid="captured-image"
          />
        ) : (
          <video
            ref={videoRef}
            autoPlay
            playsInline
            muted
            className={`w-full h-full object-cover ${!isStreaming ? 'hidden' : ''}`}
            data-testid="camera-video"
          />
        )}

        <canvas ref={canvasRef} className="hidden" />

        {error && (
          <div className="absolute inset-0 flex items-center justify-center bg-red-900/80 text-white p-4 text-center">
            <div>
              <p className="font-semibold mb-2">Camera Error</p>
              <p className="text-sm">{error}</p>
            </div>
          </div>
        )}
      </div>

      <div className="flex gap-3">
        {!isStreaming && !capturedImage && (
          <button
            onClick={startCamera}
            className="px-6 py-3 bg-blue-600 hover:bg-blue-700 rounded-lg text-white font-medium transition-colors flex items-center gap-2"
            data-testid="start-camera-button"
          >
            <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z" />
            </svg>
            Start Camera
          </button>
        )}

        {isStreaming && !capturedImage && (
          <>
            <button
              onClick={captureImage}
              className="px-6 py-3 bg-green-600 hover:bg-green-700 rounded-lg text-white font-medium transition-colors flex items-center gap-2"
              data-testid="capture-button"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 9a2 2 0 012-2h.93a2 2 0 001.664-.89l.812-1.22A2 2 0 0110.07 4h3.86a2 2 0 011.664.89l.812 1.22A2 2 0 0018.07 7H19a2 2 0 012 2v9a2 2 0 01-2 2H5a2 2 0 01-2-2V9z" />
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 13a3 3 0 11-6 0 3 3 0 016 0z" />
              </svg>
              Capture
            </button>
            
            <button
              onClick={toggleCamera}
              className="px-4 py-3 bg-gray-600 hover:bg-gray-700 rounded-lg text-white transition-colors"
              title="Switch Camera"
              data-testid="switch-camera-button"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
              </svg>
            </button>

            <button
              onClick={stopCamera}
              className="px-4 py-3 bg-red-600 hover:bg-red-700 rounded-lg text-white transition-colors"
              title="Stop Camera"
              data-testid="stop-camera-button"
            >
              <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </>
        )}

        {capturedImage && (
          <>
            <button
              onClick={retake}
              className="px-6 py-3 bg-gray-600 hover:bg-gray-700 rounded-lg text-white font-medium transition-colors"
              data-testid="retake-button"
            >
              Retake
            </button>
          </>
        )}
      </div>
    </div>
  );
}
