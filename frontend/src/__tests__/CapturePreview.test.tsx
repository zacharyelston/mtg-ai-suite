import { render, screen, fireEvent } from '@testing-library/react';
import { CapturePreview } from '@/components/CapturePreview';
import { CaptureMetadata, CaptureResponse } from '@/types/capture';

const mockImageData = 'data:image/jpeg;base64,/9j/4AAQSkZJRg==';

const mockMetadata: CaptureMetadata = {
  capturedAt: '2024-01-15T10:30:00.000Z',
  facingMode: 'environment',
  resolution: {
    width: 1920,
    height: 1080,
  },
};

const mockResponse: CaptureResponse = {
  success: true,
  data: {
    captureId: 'test-capture-123',
    status: 'completed',
    serverRecognition: {
      cardId: 'card-456',
      cardName: 'Lightning Bolt',
      confidence: 0.95,
    },
  },
};

describe('CapturePreview', () => {
  it('renders capture preview component', () => {
    render(
      <CapturePreview
        imageData={mockImageData}
        metadata={mockMetadata}
        onSubmit={jest.fn()}
        onDiscard={jest.fn()}
      />
    );
    
    expect(screen.getByTestId('capture-preview')).toBeInTheDocument();
  });

  it('displays captured image', () => {
    render(
      <CapturePreview
        imageData={mockImageData}
        metadata={mockMetadata}
        onSubmit={jest.fn()}
        onDiscard={jest.fn()}
      />
    );
    
    const image = screen.getByAltText('Captured card');
    expect(image).toBeInTheDocument();
    expect(image).toHaveAttribute('src', mockImageData);
  });

  it('displays capture metadata', () => {
    render(
      <CapturePreview
        imageData={mockImageData}
        metadata={mockMetadata}
        onSubmit={jest.fn()}
        onDiscard={jest.fn()}
      />
    );
    
    expect(screen.getByText('Capture Details')).toBeInTheDocument();
    expect(screen.getByText('1920 x 1080')).toBeInTheDocument();
    expect(screen.getByText('Back')).toBeInTheDocument();
  });

  it('shows front camera label when facingMode is user', () => {
    const frontMetadata: CaptureMetadata = {
      ...mockMetadata,
      facingMode: 'user',
    };

    render(
      <CapturePreview
        imageData={mockImageData}
        metadata={frontMetadata}
        onSubmit={jest.fn()}
        onDiscard={jest.fn()}
      />
    );
    
    expect(screen.getByText('Front')).toBeInTheDocument();
  });

  it('shows submit button when no response', () => {
    render(
      <CapturePreview
        imageData={mockImageData}
        metadata={mockMetadata}
        onSubmit={jest.fn()}
        onDiscard={jest.fn()}
      />
    );
    
    expect(screen.getByTestId('submit-capture-button')).toBeInTheDocument();
    expect(screen.getByText('Submit for Recognition')).toBeInTheDocument();
  });

  it('shows submitting state when isSubmitting is true', () => {
    render(
      <CapturePreview
        imageData={mockImageData}
        metadata={mockMetadata}
        isSubmitting={true}
        onSubmit={jest.fn()}
        onDiscard={jest.fn()}
      />
    );
    
    expect(screen.getByText('Submitting...')).toBeInTheDocument();
    expect(screen.getByTestId('submit-capture-button')).toBeDisabled();
  });

  it('calls onSubmit when submit button is clicked', () => {
    const mockOnSubmit = jest.fn();
    
    render(
      <CapturePreview
        imageData={mockImageData}
        metadata={mockMetadata}
        onSubmit={mockOnSubmit}
        onDiscard={jest.fn()}
      />
    );
    
    fireEvent.click(screen.getByTestId('submit-capture-button'));
    expect(mockOnSubmit).toHaveBeenCalledTimes(1);
  });

  it('calls onDiscard when discard button is clicked', () => {
    const mockOnDiscard = jest.fn();
    
    render(
      <CapturePreview
        imageData={mockImageData}
        metadata={mockMetadata}
        onSubmit={jest.fn()}
        onDiscard={mockOnDiscard}
      />
    );
    
    fireEvent.click(screen.getByTestId('discard-button'));
    expect(mockOnDiscard).toHaveBeenCalledTimes(1);
  });

  it('shows Discard button text when no response', () => {
    render(
      <CapturePreview
        imageData={mockImageData}
        metadata={mockMetadata}
        onSubmit={jest.fn()}
        onDiscard={jest.fn()}
      />
    );
    
    expect(screen.getByText('Discard')).toBeInTheDocument();
  });

  it('shows Close button text when response exists', () => {
    render(
      <CapturePreview
        imageData={mockImageData}
        metadata={mockMetadata}
        response={mockResponse}
        onSubmit={jest.fn()}
        onDiscard={jest.fn()}
      />
    );
    
    expect(screen.getByText('Close')).toBeInTheDocument();
  });

  it('hides submit button when response exists', () => {
    render(
      <CapturePreview
        imageData={mockImageData}
        metadata={mockMetadata}
        response={mockResponse}
        onSubmit={jest.fn()}
        onDiscard={jest.fn()}
      />
    );
    
    expect(screen.queryByTestId('submit-capture-button')).not.toBeInTheDocument();
  });

  it('displays recognition result when response has serverRecognition', () => {
    render(
      <CapturePreview
        imageData={mockImageData}
        metadata={mockMetadata}
        response={mockResponse}
        onSubmit={jest.fn()}
        onDiscard={jest.fn()}
      />
    );
    
    expect(screen.getByText('Detected: Lightning Bolt')).toBeInTheDocument();
    expect(screen.getByText('Confidence: 95.0%')).toBeInTheDocument();
  });

  it('displays status from response', () => {
    render(
      <CapturePreview
        imageData={mockImageData}
        metadata={mockMetadata}
        response={mockResponse}
        onSubmit={jest.fn()}
        onDiscard={jest.fn()}
      />
    );
    
    expect(screen.getByText('completed')).toBeInTheDocument();
  });
});
