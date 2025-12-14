import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { CameraCapture } from '@/components/CameraCapture';

const mockGetUserMedia = jest.fn();
const mockPlay = jest.fn().mockResolvedValue(undefined);
const mockStop = jest.fn();

Object.defineProperty(global.navigator, 'mediaDevices', {
  value: {
    getUserMedia: mockGetUserMedia,
  },
  writable: true,
});

describe('CameraCapture', () => {
  beforeEach(() => {
    jest.clearAllMocks();
    
    mockGetUserMedia.mockResolvedValue({
      getTracks: () => [{ stop: mockStop }],
    });
  });

  it('renders camera capture component', () => {
    const mockOnCapture = jest.fn();
    render(<CameraCapture onCapture={mockOnCapture} />);
    
    expect(screen.getByTestId('camera-capture')).toBeInTheDocument();
  });

  it('shows start camera button initially', () => {
    const mockOnCapture = jest.fn();
    render(<CameraCapture onCapture={mockOnCapture} />);
    
    expect(screen.getByTestId('start-camera-button')).toBeInTheDocument();
    expect(screen.getByText('Start Camera')).toBeInTheDocument();
  });

  it('shows camera not started message initially', () => {
    const mockOnCapture = jest.fn();
    render(<CameraCapture onCapture={mockOnCapture} />);
    
    expect(screen.getByText('Camera not started')).toBeInTheDocument();
  });

  it('calls getUserMedia when start camera is clicked', async () => {
    const mockOnCapture = jest.fn();
    
    const mockStream = {
      getTracks: () => [{ stop: mockStop }],
    };
    mockGetUserMedia.mockResolvedValue(mockStream);

    render(<CameraCapture onCapture={mockOnCapture} />);
    
    const startButton = screen.getByTestId('start-camera-button');
    fireEvent.click(startButton);

    await waitFor(() => {
      expect(mockGetUserMedia).toHaveBeenCalledWith({
        video: {
          facingMode: 'environment',
          width: { ideal: 1920 },
          height: { ideal: 1080 },
        },
        audio: false,
      });
    });
  });

  it('calls onError when camera access fails', async () => {
    const mockOnCapture = jest.fn();
    const mockOnError = jest.fn();
    
    mockGetUserMedia.mockRejectedValue(new Error('Camera access denied'));

    render(<CameraCapture onCapture={mockOnCapture} onError={mockOnError} />);
    
    const startButton = screen.getByTestId('start-camera-button');
    fireEvent.click(startButton);

    await waitFor(() => {
      expect(mockOnError).toHaveBeenCalledWith('Camera access denied');
    });
  });

  it('displays error message when camera access fails', async () => {
    const mockOnCapture = jest.fn();
    
    mockGetUserMedia.mockRejectedValue(new Error('Permission denied'));

    render(<CameraCapture onCapture={mockOnCapture} />);
    
    const startButton = screen.getByTestId('start-camera-button');
    fireEvent.click(startButton);

    await waitFor(() => {
      expect(screen.getByText('Camera Error')).toBeInTheDocument();
      expect(screen.getByText('Permission denied')).toBeInTheDocument();
    });
  });
});
