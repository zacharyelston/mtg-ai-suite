export interface CaptureMetadata {
  capturedAt: string;
  deviceId?: string;
  facingMode?: 'user' | 'environment';
  resolution?: {
    width: number;
    height: number;
  };
}

export interface ClientRecognition {
  cardName?: string;
  confidence?: number;
  ocrRaw?: string;
}

export interface CaptureRequest {
  image: string;
  metadata: CaptureMetadata;
  clientRecognition?: ClientRecognition;
}

export interface ServerRecognition {
  cardId: string;
  cardName: string;
  confidence: number;
}

export interface CaptureResponse {
  success: boolean;
  data: {
    captureId: string;
    serverRecognition?: ServerRecognition;
    status: 'queued' | 'processing' | 'completed' | 'failed';
  };
}

export interface CaptureState {
  isCapturing: boolean;
  lastCapture?: {
    imageData: string;
    metadata: CaptureMetadata;
    response?: CaptureResponse;
  };
  error?: string;
}
