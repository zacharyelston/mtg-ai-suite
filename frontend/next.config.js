/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  images: {
    domains: ['cards.scryfall.io', 'c1.scryfall.com'],
  },
}

module.exports = nextConfig
