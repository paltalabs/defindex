/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  
  transpilePackages: [
    '@creit.tech/stellar-wallets-kit',
    'stellar-react',
    'lit',
    '@lit-labs/react',
    '@lit/reactive-element'
  ],
  //Gregemax:main
  webpack: (config, {isServer}) => {
    // Enable top-level await
    config.experiments = {
      ...config.experiments,
      topLevelAwait: true,
    };

    // Fix for ES Module/CommonJS interop issues
    config.resolve.extensionAlias = {
      ...config.resolve.extensionAlias,
      '.js': ['.js', '.ts', '.tsx']
    };

    // Add specific handling for the problematic package
    config.module = {
      ...config.module,
      rules: [
        ...config.module.rules,
        {
          test: /\.m?js$/,
          include: [
            /node_modules\/@creit\.tech\/stellar-wallets-kit/,
            /node_modules\/lit/,
          ],
          type: 'javascript/auto',
          resolve: {
            fullySpecified: false,
          },
        },
      ],
    };

    if (!isServer) {
      config.resolve.fallback = {
        ...config.resolve.fallback,
        fs: false,
        net: false,
        tls: false,
      };
    }
    
    return config;
  },

};

export default nextConfig;
