export function SoftwareApplicationSchema() {
  const schema = {
    "@context": "https://schema.org",
    "@type": "SoftwareApplication",
    "name": "DeFindex SDK",
    "applicationCategory": "DeveloperApplication",
    "operatingSystem": "Stellar Blockchain",
    "description": "Yield infrastructure SDK for Stellar wallets and DeFi applications. Automated stablecoin vault SDKs built on Soroban with 80% revenue share.",
    "offers": {
      "@type": "Offer",
      "price": "0",
      "priceCurrency": "USD",
      "description": "Free to integrate with revenue sharing model"
    },
    "provider": {
      "@type": "Organization",
      "name": "DeFindex",
      "url": "https://defindex.io"
    },
    "featureList": [
      "Automated yield strategies",
      "Stablecoin vault management",
      "Wallet integration SDK",
      "80% revenue share",
      "Built on Soroban smart contracts"
    ],
    "keywords": "Stellar, Soroban, DeFi, yield, stablecoin, vaults, SDK, wallet integration, blockchain"
  };

  return (
    <script
      type="application/ld+json"
      dangerouslySetInnerHTML={{ __html: JSON.stringify(schema) }}
    />
  );
}

export function OrganizationSchema() {
  const schema = {
    "@context": "https://schema.org",
    "@type": "Organization",
    "name": "DeFindex",
    "url": "https://defindex.io",
    "logo": "https://defindex.io/images/logo.png",
    "description": "Leading yield infrastructure provider for Stellar wallets and DeFi applications. Built on Soroban smart contracts.",
    "foundingDate": "2023",
    "sameAs": [
      "https://twitter.com/defindex_",
      "https://github.com/paltalabs/defindex",
      "https://discord.gg/CUC26qUTw7",
      "https://www.linkedin.com/company/defindex"
    ],
    "contactPoint": {
      "@type": "ContactPoint",
      "email": "dev@paltalabs.io",
      "contactType": "Customer Support"
    }
  };

  return (
    <script
      type="application/ld+json"
      dangerouslySetInnerHTML={{ __html: JSON.stringify(schema) }}
    />
  );
}

export function WebSiteSchema() {
  const schema = {
    "@context": "https://schema.org",
    "@type": "WebSite",
    "name": "DeFindex",
    "url": "https://defindex.io",
    "description": "Stellar yield infrastructure for wallets and DeFi apps",
    "potentialAction": {
      "@type": "SearchAction",
      "target": "https://defindex.io/search?q={search_term_string}",
      "query-input": "required name=search_term_string"
    }
  };

  return (
    <script
      type="application/ld+json"
      dangerouslySetInnerHTML={{ __html: JSON.stringify(schema) }}
    />
  );
}
