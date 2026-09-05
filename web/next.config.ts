import type { NextConfig } from 'next';

const nextConfig: NextConfig = {
  reactStrictMode: true,

  async redirects() {
    return [
      { source: '/pot-odds-equity', destination: '/pot-odds', permanent: true }
    ];
  }
};

export default nextConfig;
