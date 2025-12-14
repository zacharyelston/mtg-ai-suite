'use client';

import { CaptureMetadata, CaptureResponse } from '@/types/capture';

interface CapturePreviewProps {
  imageData: string;
  metadata: CaptureMetadata;
  response?: CaptureResponse;
  isSubmitting?: boolean;
  onSubmit: () => void;
  onDiscard: () => void;
}

export function CapturePreview({
  imageData,
  metadata,
  response,
  isSubmitting,
  onSubmit,
  onDiscard,
}: CapturePreviewProps) {
  return (
    <div className="flex flex-col md:flex-row gap-6 p-4 bg-gray-800 rounded-lg" data-testid="capture-preview">
      <div className="flex-shrink-0">
        <img
          src={imageData}
          alt="Captured card"
          className="w-full max-w-sm rounded-lg"
        />
      </div>

      <div className="flex-grow text-white">
        <h3 className="text-xl font-semibold mb-4">Capture Details</h3>

        <div className="space-y-2 text-sm">
          <p>
            <span className="text-gray-400">Captured:</span>{' '}
            {new Date(metadata.capturedAt).toLocaleString()}
          </p>
          {metadata.resolution && (
            <p>
              <span className="text-gray-400">Resolution:</span>{' '}
              {metadata.resolution.width} x {metadata.resolution.height}
            </p>
          )}
          <p>
            <span className="text-gray-400">Camera:</span>{' '}
            {metadata.facingMode === 'environment' ? 'Back' : 'Front'}
          </p>
        </div>

        {response && (
          <div className="mt-4 p-3 bg-gray-700 rounded">
            <p className="text-sm">
              <span className="text-gray-400">Status:</span>{' '}
              <span className={
                response.data.status === 'completed' ? 'text-green-400' :
                response.data.status === 'failed' ? 'text-red-400' :
                'text-yellow-400'
              }>
                {response.data.status}
              </span>
            </p>
            {response.data.serverRecognition && (
              <div className="mt-2">
                <p className="text-green-400 font-medium">
                  Detected: {response.data.serverRecognition.cardName}
                </p>
                <p className="text-sm text-gray-400">
                  Confidence: {(response.data.serverRecognition.confidence * 100).toFixed(1)}%
                </p>
              </div>
            )}
          </div>
        )}

        <div className="flex gap-3 mt-6">
          {!response && (
            <button
              onClick={onSubmit}
              disabled={isSubmitting}
              className="px-6 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-800 rounded text-white font-medium transition-colors"
              data-testid="submit-capture-button"
            >
              {isSubmitting ? 'Submitting...' : 'Submit for Recognition'}
            </button>
          )}
          <button
            onClick={onDiscard}
            className="px-6 py-2 bg-gray-600 hover:bg-gray-700 rounded text-white font-medium transition-colors"
            data-testid="discard-button"
          >
            {response ? 'Close' : 'Discard'}
          </button>
        </div>
      </div>
    </div>
  );
}
