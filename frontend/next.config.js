/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  images: {
    domains: ['cards.scryfall.io', 'c1.scryfall.com'],
  },
  allowedDevOrigins: ['*.replit.dev', '*.replit.app', '*.riker.replit.dev'],
  async headers() {
    return [
      {
        source: '/:path*',
        headers: [
          {
            key: 'Access-Control-Allow-Origin',
            value: '*',
          },
        ],
      },
    ];
  },
}

module.exports = nextConfig
